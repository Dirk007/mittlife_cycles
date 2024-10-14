use anyhow::anyhow;
use poem::http::HeaderMap;

use super::headers::{
    MappedHeaders, MARKETPLACE_HEADER_ALGORITHM, MARKETPLACE_HEADER_SERIAL, MARKETPLACE_HEADER_SIGNATURE,
};

impl TryFrom<&HeaderMap> for MappedHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let serial = extract_header(headers, MARKETPLACE_HEADER_SERIAL)?;
        let algorithm = extract_header(headers, MARKETPLACE_HEADER_ALGORITHM)?;
        let signature = extract_header(headers, MARKETPLACE_HEADER_SIGNATURE)?;

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
