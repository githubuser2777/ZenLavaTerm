use super::provider::AudioDeviceInfo;
use super::ring_buffer::PcmRingBuffer;
use crate::{LavaError, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub struct NativeAudioCapture {
    _shutdown_tx: Sender<()>,
    pub actual_sample_rate: u32,
}

impl std::fmt::Debug for NativeAudioCapture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeAudioCapture").finish()
    }
}

// Extracted unit-testable device selection logic
pub fn select_capture_device(
    host: &cpal::Host,
    device_name: Option<&str>,
    loopback: bool,
) -> Result<cpal::Device> {
    #[cfg(not(target_os = "windows"))]
    if loopback {
        return Err(LavaError::Audio(
            "Loopback capture is only supported natively on Windows".into(),
        ));
    }

    let device_opt = if let Some(name) = device_name {
        let mut found = None;
        if loopback {
            if let Ok(devices) = host.output_devices() {
                for d in devices {
                    if let Ok(n) = d.name() {
                        if n == name {
                            found = Some(d);
                            break;
                        }
                    }
                }
            }
        } else {
            if let Ok(devices) = host.input_devices() {
                for d in devices {
                    if let Ok(n) = d.name() {
                        if n == name {
                            found = Some(d);
                            break;
                        }
                    }
                }
            }
        }
        found
    } else {
        if loopback {
            host.default_output_device()
        } else {
            host.default_input_device()
        }
    };

    device_opt.ok_or_else(|| LavaError::Audio("Audio capture device not found".into()))
}

// Helper to get the correct host across platforms
fn get_cpal_host() -> cpal::Host {
    #[cfg(target_os = "windows")]
    {
        // For Windows loopback, WASAPI is required.
        cpal::host_from_id(cpal::HostId::Wasapi).unwrap_or_else(|_| cpal::default_host())
    }
    #[cfg(not(target_os = "windows"))]
    {
        cpal::default_host()
    }
}

impl NativeAudioCapture {
    pub fn new(
        ring_buffer: PcmRingBuffer,
        device_name: Option<&str>,
        loopback: bool,
    ) -> Result<Self> {
        let (shutdown_tx, shutdown_rx) = channel();
        let (ready_tx, ready_rx) = channel();

        let device_name_clone = device_name.map(|s| s.to_string());

        // Fail fast if loopback on unsupported OS
        #[cfg(not(target_os = "windows"))]
        if loopback {
            return Err(LavaError::Audio(
                "Loopback capture is only supported natively on Windows".into(),
            ));
        }

        thread::Builder::new()
            .name("cpal_audio_worker".into())
            .spawn(move || {
                let host = get_cpal_host();
                let device =
                    match select_capture_device(&host, device_name_clone.as_deref(), loopback) {
                        Ok(d) => d,
                        Err(e) => {
                            let _ = ready_tx.send(Err(e));
                            return;
                        }
                    };

                // For output/loopback devices, they do not have an input config.
                // We must use their output config, and CPAL's WASAPI backend natively injects
                // AUDCLNT_STREAMFLAGS_LOOPBACK when building an input stream on an eRender device.
                let supported_config = match device
                    .default_input_config()
                    .or_else(|_| device.default_output_config())
                {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = ready_tx.send(Err(LavaError::Audio(format!(
                            "Failed to get device config: {}",
                            e
                        ))));
                        return;
                    }
                };

                let sample_format = supported_config.sample_format();
                let config: StreamConfig = supported_config.into();
                let channels = config.channels as usize;
                let sample_rate = config.sample_rate.0;

                let err_fn = |err| eprintln!("Audio stream error: {}", err);

                let stream_res = match sample_format {
                    SampleFormat::F32 => {
                        let rb = ring_buffer.clone();
                        device.build_input_stream(
                            &config,
                            move |data: &[f32], _| rb.push_interleaved_f32(data, channels),
                            err_fn,
                            None,
                        )
                    }
                    SampleFormat::I16 => {
                        let rb = ring_buffer.clone();
                        device.build_input_stream(
                            &config,
                            move |data: &[i16], _| rb.push_interleaved_i16(data, channels),
                            err_fn,
                            None,
                        )
                    }
                    SampleFormat::U16 => {
                        let rb = ring_buffer.clone();
                        device.build_input_stream(
                            &config,
                            move |data: &[u16], _| rb.push_interleaved_u16(data, channels),
                            err_fn,
                            None,
                        )
                    }
                    _ => {
                        let _ = ready_tx
                            .send(Err(LavaError::Audio("Unsupported sample format".into())));
                        return;
                    }
                };

                match stream_res {
                    Ok(stream) => {
                        if let Err(e) = stream.play() {
                            let _ = ready_tx.send(Err(LavaError::Audio(format!(
                                "Failed to start stream: {}",
                                e
                            ))));
                            return;
                        }

                        if ready_tx.send(Ok(sample_rate)).is_ok() {
                            let _ = shutdown_rx.recv();
                        }
                    }
                    Err(e) => {
                        let _ = ready_tx.send(Err(LavaError::Audio(format!(
                            "Failed to build stream: {}",
                            e
                        ))));
                    }
                }
            })
            .map_err(|e| LavaError::Audio(format!("Failed to spawn audio thread: {}", e)))?;

        let actual_sample_rate = ready_rx.recv().map_err(|_| {
            LavaError::Audio("Audio worker thread panicked during initialization".into())
        })??;

        Ok(Self {
            _shutdown_tx: shutdown_tx,
            actual_sample_rate,
        })
    }

    pub fn list_devices() -> Vec<AudioDeviceInfo> {
        let mut devices = Vec::new();
        let host = get_cpal_host();

        let default_in = host.default_input_device().and_then(|d| d.name().ok());

        #[cfg(target_os = "windows")]
        let default_out = host.default_output_device().and_then(|d| d.name().ok());
        #[cfg(not(target_os = "windows"))]
        let _default_out: Option<String> = None;

        if let Ok(input_devices) = host.input_devices() {
            for d in input_devices {
                if let Ok(name) = d.name() {
                    let is_default = Some(&name) == default_in.as_ref();
                    if !devices
                        .iter()
                        .any(|existing: &AudioDeviceInfo| existing.name == name)
                    {
                        devices.push(AudioDeviceInfo {
                            name,
                            is_default,
                            direction: "input",
                        });
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        if let Ok(output_devices) = host.output_devices() {
            for d in output_devices {
                if let Ok(name) = d.name() {
                    if !devices
                        .iter()
                        .any(|existing: &AudioDeviceInfo| existing.name == name)
                    {
                        let is_default = Some(&name) == default_out.as_ref();
                        devices.push(AudioDeviceInfo {
                            name,
                            is_default,
                            direction: "output",
                        });
                    }
                }
            }
        }

        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices_does_not_panic() {
        let devices = NativeAudioCapture::list_devices();
        if !devices.is_empty() {
            println!("Found {} devices", devices.len());
        }
    }

    #[test]
    fn test_select_capture_device_invalid_name() {
        let host = get_cpal_host();
        let res = select_capture_device(&host, Some("ThisDeviceDoesNotExist12345"), false);
        assert!(res.is_err());
        match res {
            Err(err) => match err {
                LavaError::Audio(msg) => assert!(msg.contains("Audio capture device not found")),
                _ => panic!("Expected Audio error"),
            },
            Ok(_) => panic!("Expected error, got Ok"),
        }
    }

    #[test]
    fn test_select_capture_device_loopback_os_support() {
        let host = get_cpal_host();
        let res = select_capture_device(&host, None, true);

        #[cfg(not(target_os = "windows"))]
        {
            assert!(res.is_err());
            match res {
                Err(err) => match err {
                    LavaError::Audio(msg) => {
                        assert!(
                            msg.contains("Loopback capture is only supported natively on Windows")
                        )
                    }
                    _ => panic!("Expected Audio error"),
                },
                Ok(_) => panic!("Expected error, got Ok"),
            }
        }

        // On Windows it might actually succeed if the test runner has an output device,
        // or fail gracefully if it doesn't. We just ensure it doesn't panic.
        #[cfg(target_os = "windows")]
        {
            let _ = res;
        }
    }
}
