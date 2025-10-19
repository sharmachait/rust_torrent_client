use serde_json;
use std::env;
use clap::builder::TypedValueParser;
use codecrafters_bittorrent::{decode_bencoded_ints, decode_bencoded_strings};
use serde_json::Value;

// Available if you need it!
// use serde_bencode
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
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
        Some('0'..='9')=>{
            decode_bencoded_strings(encoded_value)
        }
        _=>{
            panic!("not encoded properly: {}", encoded_value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);

        println!("{}", decoded_value.0.to_string());

    } else {
        println!("unknown command: {}", args[1])
    }
}
