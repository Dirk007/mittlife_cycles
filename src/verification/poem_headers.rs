use anyhow::{anyhow, Result};
#[cfg(feature = "poemheader")]
use poem::http::HeaderMap;

use super::headers::MappedHeaders;

impl TryFrom<&HeaderMap> for MappedHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: &HeaderMap) -> std::result::Result<Self, Self::Error> {
        let serial = extract_heaader(headers, "X-Marketplace-Signature-Serial")?;
        let algorithm = extract_heaader(headers, "X-Marketplace-Signature-Algorithm")?;
        let signature = extract_heaader(headers, "X-Marketplace-Signature")?;

        Ok(MappedHeaders {
            signature: signature.to_string(),
            algorithm: algorithm.to_string(),
            serial: serial.to_string(),
        })
    }
}

fn extract_heaader<'a>(headers: &'a HeaderMap, key: &str) -> Result<&'a str> {
    let result = headers.get(key).ok_or(anyhow!("Missing {} header", key))?.to_str()?;
    Ok(result)
}
