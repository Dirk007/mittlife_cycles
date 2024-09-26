use actix_web::http::header::HeaderMap;
use anyhow::anyhow;

use super::MappedHeaders;

impl TryFrom<&HeaderMap> for MappedHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let serial = extract_header(headers, "X-Marketplace-Signature-Serial")?;
        let algorithm = extract_header(headers, "X-Marketplace-Signature-Serial")?;
        let signature = extract_header(headers, "X-Marketplace-Signature-Serial")?;

        Ok(MappedHeaders {
            serial,
            algorithm,
            signature,
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
