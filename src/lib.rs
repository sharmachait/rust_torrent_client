pub fn decode_bencoded_strings(encoded_value: &str) -> serde_json::Value{
    let split = encoded_value.split_once(':');
    match split {
        Some((len,string)) => {
            let length= len.parse::<usize>();
            match length {
                Ok(parsedLength) => {
                    let encoded = &(string[..parsedLength]);
                    serde_json::Value::String(encoded.to_string())
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
    let val = encoded_value.parse::<usize>();
    match val {
        Ok(v) => {
            serde_json::Value::String(v.to_string())
        },
        Err(e) => {
            eprintln!("{}",e);
            panic!("not a number: {}", encoded_value)
        }
    }
}