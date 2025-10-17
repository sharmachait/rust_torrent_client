use serde_json;
use std::env;
use codecrafters_bittorrent::{decode_bencoded_ints, decode_bencoded_strings};
// Available if you need it!
// use serde_bencode


fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if let Some(rest) = encoded_value.strip_prefix('i') {
        if let Some(inner) = rest.strip_suffix('e') {
            decode_bencoded_ints(inner)
        }else{
            panic!("not encoded properly: {}", encoded_value)
        }
    }else{
        decode_bencoded_strings(encoded_value)
    }
}


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
