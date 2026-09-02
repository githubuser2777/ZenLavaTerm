import re
with open('src/audio/mod.rs', 'r') as f:
    c = f.read()

c = c.replace('let live_provider = create_live_audio_provider(config.device.as_deref())?;', 'let live_provider = create_live_audio_provider(config.device.as_deref(), config.loopback)?;')
c = c.replace('pub fn create_live_audio_provider(device_name: Option<&str>) -> Result<LiveAudioProvider> {', 'pub fn create_live_audio_provider(device_name: Option<&str>, loopback: bool) -> Result<LiveAudioProvider> {')
c = c.replace('let capture = native::NativeAudioCapture::new(ring_buffer.clone(), device_name)?;', 'let capture = native::NativeAudioCapture::new(ring_buffer.clone(), device_name, loopback)?;')

with open('src/audio/mod.rs', 'w') as f:
    f.write(c)
