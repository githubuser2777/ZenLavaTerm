with open('src/audio/native.rs', 'r') as f:
    c = f.read()

c = c.replace('let default_out: Option<String> = None;', 'let _default_out: Option<String> = None;')
c = c.replace('let err = res.unwrap_err();\n        match err {', 'match res {\n            Err(err) => match err {')
c = c.replace('            _ => panic!("Expected Audio error"),\n        }', '            _ => panic!("Expected Audio error"),\n            },\n            Ok(_) => panic!("Expected error, got Ok"),\n        }')
c = c.replace('            let err = res.unwrap_err();\n            match err {', '            match res {\n                Err(err) => match err {')
c = c.replace('                _ => panic!("Expected Audio error"),\n            }', '                _ => panic!("Expected Audio error"),\n                },\n                Ok(_) => panic!("Expected error, got Ok"),\n            }')

with open('src/audio/native.rs', 'w') as f:
    f.write(c)
