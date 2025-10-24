use std::fmt;
use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;

#[derive(Clone, Debug)]
pub struct Hashes(Vec<[u8;20]>); // this defines a new data type that is essentially just a tuple with a single element of type Vec<[u8;20]>
struct HashesVisitor;

impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte string whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.len() %20 != 0 {
            return Err(E::custom(format!("length is {}", value.len())));
        }

        Ok(
            Hashes(
                value
                    .chunks_exact(20)
                    .map(
                        |slice|{
                            slice.try_into().expect("guaranteed to be length 20")
                        }
                    ).collect()
            )
        )
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(HashesVisitor)
    }
}