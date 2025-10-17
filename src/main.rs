use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {

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

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
