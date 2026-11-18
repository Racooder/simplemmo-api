use serde::{Deserialize, Deserializer, de::Error as _};

pub(crate) fn bool_from_int<'de, D>(d: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(d)? {
        0 => Ok(false),
        1 => Ok(true),
        n => Err(D::Error::custom(format!("expected 0 or 1, got {n}"))),
    }
}

pub(crate) fn bool_from_nonzero<'de, D>(d: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(d)? {
        0 => Ok(false),
        _ => Ok(true),
    }
}
