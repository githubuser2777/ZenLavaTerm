import re
with open('src/audio/native.rs', 'r') as f:
    c = f.read()

c = c.replace('pub fn new(ring_buffer: PcmRingBuffer, device_name: Option<&str>) -> Result<Self> {', 'pub fn new(ring_buffer: PcmRingBuffer, device_name: Option<&str>, loopback: bool) -> Result<Self> {')
c = c.replace('let device_name_clone = device_name.map(|s| s.to_string());', 'let device_name_clone = device_name.map(|s| s.to_string());\n        let loopback_clone = loopback;')

device_opt_logic = """
                let device_opt = if let Some(ref name) = device_name_clone {
                    let mut found = None;
                    if loopback_clone && cfg!(target_os = "windows") {
                        if let Ok(devices) = host.output_devices() {
                            for d in devices {
                                if let Ok(n) = d.name() {
                                    if &n == name { found = Some(d); break; }
                                }
                            }
                        }
                    } else {
                        if let Ok(devices) = host.input_devices() {
                            for d in devices {
                                if let Ok(n) = d.name() {
                                    if &n == name { found = Some(d); break; }
                                }
                            }
                        }
                    }
                    found
                } else {
                    if loopback_clone && cfg!(target_os = "windows") {
                        host.default_output_device()
                    } else {
                        host.default_input_device()
                    }
                };
"""

c = re.sub(r'                let device_opt = if let Some\(ref name\) = device_name_clone \{.*?\n                \};\n', device_opt_logic, c, flags=re.DOTALL)

with open('src/audio/native.rs', 'w') as f:
    f.write(c)
