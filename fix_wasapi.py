import re
with open('src/audio/native.rs', 'r') as f:
    c = f.read()

c = c.replace('let host = cpal::default_host();', '''#[cfg(target_os = "windows")]
                let host = cpal::host_from_id(cpal::HostId::Wasapi).unwrap_or_else(|_| cpal::default_host());
                #[cfg(not(target_os = "windows"))]
                let host = cpal::default_host();''')

# Add the ponytail comment
comment = """
                // ponytail: cpal natively injects AUDCLNT_STREAMFLAGS_LOOPBACK when build_input_stream is called on an eRender device (see cpal/src/host/wasapi/device.rs). Skipped separate wasapi crate.
                let supported_config = match device.default_input_config().or_else(|_| device.default_output_config()) {"""

c = c.replace('                let supported_config = match device.default_input_config().or_else(|_| device.default_output_config()) {', comment)

with open('src/audio/native.rs', 'w') as f:
    f.write(c)
