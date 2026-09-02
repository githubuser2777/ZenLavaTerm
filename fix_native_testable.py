with open('src/audio/native.rs', 'r') as f:
    lines = f.readlines()

new_lines = []
in_spawn = False
skip = False
for line in lines:
    if 'use super::provider::AudioDeviceInfo;' in line:
        new_lines.append(line)
        new_lines.append("\n// Extracted unit-testable device selection logic\n")
        new_lines.append("fn select_capture_device(host: &cpal::Host, device_name: Option<&str>, loopback: bool) -> Result<cpal::Device> {\n")
        new_lines.append("    #[cfg(not(target_os = \"windows\"))]\n")
        new_lines.append("    if loopback {\n")
        new_lines.append("        return Err(LavaError::Audio(\"Loopback capture is only supported natively on Windows\".into()));\n")
        new_lines.append("    }\n")
        new_lines.append("    let device_opt = if let Some(name) = device_name {\n")
        new_lines.append("        let mut found = None;\n")
        new_lines.append("        if loopback {\n")
        new_lines.append("            if let Ok(devices) = host.output_devices() {\n")
        new_lines.append("                for d in devices { if let Ok(n) = d.name() { if n == name { found = Some(d); break; } } }\n")
        new_lines.append("            }\n")
        new_lines.append("        } else {\n")
        new_lines.append("            if let Ok(devices) = host.input_devices() {\n")
        new_lines.append("                for d in devices { if let Ok(n) = d.name() { if n == name { found = Some(d); break; } } }\n")
        new_lines.append("            }\n")
        new_lines.append("        }\n")
        new_lines.append("        found\n")
        new_lines.append("    } else {\n")
        new_lines.append("        if loopback { host.default_output_device() } else { host.default_input_device() }\n")
        new_lines.append("    };\n")
        new_lines.append("    device_opt.ok_or_else(|| LavaError::Audio(\"Audio capture device not found\".into()))\n")
        new_lines.append("}\n")
        continue

    if '        #[cfg(not(target_os = "windows"))]' in line and 'if loopback {' in lines[lines.index(line)+1]:
        skip = True
        continue
    
    if skip and '        thread::Builder::new()' in line:
        skip = False

    if skip:
        continue
        
    if '.spawn(move || {' in line:
        new_lines.append(line)
        in_spawn = True
        skip = True
        new_lines.append('                let host = get_cpal_host();\n')
        new_lines.append('                let device = match select_capture_device(&host, device_name_clone.as_deref(), loopback) {\n')
        new_lines.append('                    Ok(d) => d,\n')
        new_lines.append('                    Err(e) => { let _ = ready_tx.send(Err(e)); return; }\n')
        new_lines.append('                };\n')
        continue
        
    if in_spawn and skip and '                let supported_config = match device' in line:
        skip = False
        in_spawn = False
        new_lines.append(line)
        continue
        
    if not skip:
        new_lines.append(line)

with open('src/audio/native.rs', 'w') as f:
    f.writelines(new_lines)
