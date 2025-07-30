use std::io::Read;

use crate::cli::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
