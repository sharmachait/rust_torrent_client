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
