use anyhow::anyhow;
use poem::http::HeaderMap;

use super::headers::MappedHeaders;

impl TryFrom<&HeaderMap> for MappedHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let serial = extract_header(headers, "X-Marketplace-Signature-Serial")?;
        let algorithm = extract_header(headers, "X-Marketplace-Signature-Algorithm")?;
        let signature = extract_header(headers, "X-Marketplace-Signature")?;

        Ok(MappedHeaders {
            signature,
            algorithm,
            serial,
        })
    }
}

fn extract_header(headers: &HeaderMap, key: &str) -> anyhow::Result<String> {
    Ok(headers
        .get(key)
        .ok_or(anyhow!("Missing {} header", key))?
        .to_str()?
        .to_string())
}
