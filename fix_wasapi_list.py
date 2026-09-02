import re
with open('src/audio/native.rs', 'r') as f:
    c = f.read()

c = c.replace('        let host = cpal::default_host();', '''        #[cfg(target_os = "windows")]
        let host = cpal::host_from_id(cpal::HostId::Wasapi).unwrap_or_else(|_| cpal::default_host());
        #[cfg(not(target_os = "windows"))]
        let host = cpal::default_host();''')

with open('src/audio/native.rs', 'w') as f:
    f.write(c)
