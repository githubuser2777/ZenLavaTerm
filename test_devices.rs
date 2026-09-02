use cpal::traits::{DeviceTrait, HostTrait};
fn main() {
    let host = cpal::default_host();
    if let Ok(devices) = host.input_devices() {
        for d in devices {
            println!("IN: {}", d.name().unwrap_or_default());
        }
    }
    if let Ok(devices) = host.output_devices() {
        for d in devices {
            println!("OUT: {}", d.name().unwrap_or_default());
        }
    }
}
