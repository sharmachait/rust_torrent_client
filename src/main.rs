use serde_json;
use std::path::PathBuf;
use anyhow::Context;
use codecrafters_bittorrent::{torrent};
use serde_bencode;
use clap::{Parser, Subcommand};
use serde::{Deserializer};
use codecrafters_bittorrent::torrent::Keys;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Decode {
        value: String
    },
    Info {
        torrent: PathBuf
    },
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    match args.command{
        Commands::Decode {value}=>{
            let v: serde_bencode::value::Value = serde_bencode::de::from_str(&value)
                .context("Failed to decode bencode")?;
            let json = bencode_to_json(v);
            println!("{}", json);
        },
        Commands::Info {torrent}=>{
            let mut f = std::fs::read(torrent).context("Read Torrent file")?;
            let t:torrent::Torrent = serde_bencode::from_bytes(&f).context("Parse Torrent File")?;

            
            println!("Tracker URL: {}", t.announce);
            if let Keys::SingleFile {length} = t.info.key {
                println!("Length: {:?}", length);
            }
        }
    }
    Ok(())
}

fn bencode_to_json(value: serde_bencode::value::Value) -> serde_json::Value {
    use serde_bencode::value::Value;

    match value {
        Value::Bytes(b) => {
            // Try to convert to UTF-8 string, otherwise use hex
            if let Ok(s) = String::from_utf8(b.clone()) {
                serde_json::Value::String(s)
            } else {
                serde_json::Value::String(format!("{:?}", b))
            }
        }
        Value::Int(i) => serde_json::Value::Number(i.into()),
        Value::List(l) => {
            serde_json::Value::Array(
                l.into_iter().map(bencode_to_json).collect()
            )
        }
        Value::Dict(d) => {
            let map: serde_json::Map<String, serde_json::Value> = d
                .into_iter()
                .map(|(k, v)| {
                    let key = String::from_utf8_lossy(&k).to_string();
                    (key, bencode_to_json(v))
                })
                .collect();
            serde_json::Value::Object(map)
        }
    }
}