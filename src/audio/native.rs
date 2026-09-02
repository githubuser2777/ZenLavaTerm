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

impl NativeAudioCapture {
    pub fn new(ring_buffer: PcmRingBuffer, device_name: Option<&str>) -> Result<Self> {
        let (shutdown_tx, shutdown_rx) = channel();
        let (ready_tx, ready_rx) = channel();

        let device_name_clone = device_name.map(|s| s.to_string());

        thread::Builder::new()
            .name("cpal_audio_worker".into())
            .spawn(move || {
                let host = cpal::default_host();

                let device_opt = if let Some(ref name) = device_name_clone {
                    let mut found = None;
                    if let Ok(devices) = host.input_devices() {
                        for d in devices {
                            if let Ok(n) = d.name() {
                                if &n == name {
                                    found = Some(d);
                                    break;
                                }
                            }
                        }
                    }
                    if found.is_none() {
                        if let Ok(devices) = host.output_devices() {
                            for d in devices {
                                if let Ok(n) = d.name() {
                                    if &n == name {
                                        found = Some(d);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    found
                } else {
                    host.default_input_device()
                        .or_else(|| host.default_output_device())
                };

                let device = match device_opt {
                    Some(d) => d,
                    None => {
                        let _ = ready_tx.send(Err(LavaError::Audio("Device not found".into())));
                        return;
                    }
                };

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

                        // Notify that initialization succeeded and send back the actual sample rate
                        if ready_tx.send(Ok(sample_rate)).is_ok() {
                            // Block thread until shutdown signal received
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

        // Wait for worker thread to initialize
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
        let host = cpal::default_host();

        let default_in = host.default_input_device().and_then(|d| d.name().ok());
        let default_out = host.default_output_device().and_then(|d| d.name().ok());

        if let Ok(input_devices) = host.input_devices() {
            for d in input_devices {
                if let Ok(name) = d.name() {
                    let is_default = Some(&name) == default_in.as_ref();
                    devices.push(AudioDeviceInfo { name, is_default });
                }
            }
        }
        if let Ok(output_devices) = host.output_devices() {
            for d in output_devices {
                if let Ok(name) = d.name() {
                    if !devices.iter().any(|existing| existing.name == name) {
                        let is_default = Some(&name) == default_out.as_ref();
                        devices.push(AudioDeviceInfo { name, is_default });
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
        // Just verify it runs without panicking. Depending on the CI environment,
        // it may or may not find devices.
        if !devices.is_empty() {
            println!("Found {} devices", devices.len());
        }
    }

    #[test]
    fn test_native_audio_capture_invalid_device() {
        let rb = PcmRingBuffer::new(128);
        let res = NativeAudioCapture::new(rb, Some("ThisDeviceDoesNotExist12345"));
        assert!(res.is_err());
        let err = res.unwrap_err();
        match err {
            LavaError::Audio(msg) => assert!(msg.contains("Device not found")),
            _ => panic!("Expected Audio error"),
        }
    }
}
