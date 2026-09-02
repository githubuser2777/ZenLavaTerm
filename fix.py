import re
with open('tests/integration_test.rs', 'r') as f:
    c = f.read()

c = c.replace(
    'let mut disabled_provider = create_audio_provider(&disabled_cfg);',
    'let mut disabled_provider = create_audio_provider(&disabled_cfg).unwrap();'
)
c = c.replace(
    'let mut enabled_provider = create_audio_provider(&enabled_cfg);',
    'let mut enabled_provider = create_audio_provider(&enabled_cfg).unwrap();'
)

# And reverse my previous sed replacement if it was broken
c = c.replace('disabled_provider.unwrap().is_live()', 'disabled_provider.is_live()')
c = c.replace('disabled_provider.unwrap().provider_name()', 'disabled_provider.provider_name()')
c = c.replace('disabled_provider.unwrap().poll_signals()', 'disabled_provider.poll_signals()')
c = c.replace('enabled_provider.unwrap().poll_signals()', 'enabled_provider.poll_signals()')

with open('tests/integration_test.rs', 'w') as f:
    f.write(c)
