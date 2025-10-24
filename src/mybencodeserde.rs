use serde_json::Value;

pub fn decode_bencoded_strings(encoded_value: &str) -> (serde_json::Value, &str) {
    let split = encoded_value.split_once(':');
    match split {
        Some((len,string)) => {
            let length= len.parse::<usize>();
            match length {
                Ok(parsedLength) => {
                    let encoded = &(string[..parsedLength]);
                    let left = &string[parsedLength..string.len()];
                    (encoded.into(), left)
                },
                Err(e)=>{
                    println!("{}", e);
                    panic!("Unhandled encoded value: {}", encoded_value);
                }
            }
        },
        None => panic!("not encoded properly: {}", encoded_value)
    }
}

pub fn decode_bencoded_ints(encoded_value: &str) -> serde_json::Value{
    let val = encoded_value.parse::<isize>();

    match val {
        Ok(v) => {
            v.into()
        },
        Err(e) => {
            eprintln!("{}",e);
            panic!("not a number: {}", encoded_value)
        }
    }
}

pub fn decode_bencoded_value(encoded_value: &str) -> (Value, &str) {
    match encoded_value.chars().next() {
        Some('i') => {
            let after_i = encoded_value.strip_prefix('i').unwrap();
            let split : Option<(&str, &str)> = after_i.split_once('e');
            match split {
                None => {panic!("not encoded properly: {}", encoded_value);}
                Some((num, rest)) => {
                    (decode_bencoded_ints(num), rest)
                }
            }
        },
        Some('l') =>{
            let mut rest = encoded_value.strip_prefix('l').unwrap();
            let mut values = Vec::new();
            while !rest.is_empty() &&  rest.chars().next().unwrap() != 'e' {
                let (v, remainder) = decode_bencoded_value(rest);
                values.push(v);
                rest = remainder;
            }

            assert!(!rest.is_empty());
            assert_eq!(rest.chars().next().unwrap(), 'e');
            (values.into(), &rest[1..])
        },
        Some('d') =>{
            let mut dict = serde_json::Map::new();
            let mut rest = encoded_value.strip_prefix('d').unwrap();

            while !rest.is_empty() &&  rest.chars().next().unwrap() != 'e' {
                let (k, remainder) = decode_bencoded_value(rest);
                let k = match k {
                    Value::String(x)=>x,
                    _ => {panic!("dict keys must be string not {k:?}");}
                };
                let (v, remainder) = decode_bencoded_value(remainder);
                dict.insert(k,v);
                rest = remainder;
            }

            assert!(!rest.is_empty());
            assert_eq!(rest.chars().next().unwrap(), 'e');
            (dict.into(), &rest[1..])
        },
        Some('0'..='9')=>{
            decode_bencoded_strings(encoded_value)
        }
        _=>{
            panic!("not encoded properly: {}", encoded_value);
        }
    }
}