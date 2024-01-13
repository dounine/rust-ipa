use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PageOptions {
    #[serde(default = "offset_default")]
    pub offset: u64,
    #[serde(default = "limit_default")]
    pub limit: u64,
}

impl PageOptions {
    pub fn format(&self) -> Self {
        Self {
            offset: self.offset.min(u64::MAX).max(0),
            limit: self.limit.min(20).max(1),
        }
    }
}

fn offset_default() -> u64 {
    0
}

fn limit_default() -> u64 {
    10
}

pub fn deserialize_strings_split<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let parts: Vec<String> = s.split(',').map(String::from).collect();
    Ok(parts)
}
