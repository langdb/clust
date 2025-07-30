use std::fmt::Display;

use crate::macros::impl_display_for_serialize;

/// Cache control for content blocks.
///
/// This allows for granular control over what gets cached in the API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CacheControl {
    /// The type of cache control.
    #[serde(rename = "type")]
    pub _type: CacheControlType,
}

impl Default for CacheControl {
    fn default() -> Self {
        Self {
            _type: CacheControlType::Ephemeral,
        }
    }
}

/// The type of cache control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheControlType {
    /// ephemeral - The content will be cached temporarily
    Ephemeral,
}

impl Default for CacheControlType {
    fn default() -> Self {
        Self::Ephemeral
    }
}

impl Display for CacheControlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheControlType::Ephemeral => write!(f, "ephemeral"),
        }
    }
}

impl serde::Serialize for CacheControlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for CacheControlType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ephemeral" => Ok(CacheControlType::Ephemeral),
            _ => Err(serde::de::Error::custom(format!("unknown cache control type: {}", s))),
        }
    }
}

impl_display_for_serialize!(CacheControl);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_control_serialize() {
        let cache_control = CacheControl::default();
        assert_eq!(
            serde_json::to_string(&cache_control).unwrap(),
            "{\"type\":\"ephemeral\"}"
        );
    }

    #[test]
    fn cache_control_deserialize() {
        let json = r#"{"type": "ephemeral"}"#;
        let cache_control = serde_json::from_str::<CacheControl>(json).unwrap();
        assert_eq!(cache_control._type, CacheControlType::Ephemeral);
    }

    #[test]
    fn cache_control_type_display() {
        assert_eq!(CacheControlType::Ephemeral.to_string(), "ephemeral");
    }

    #[test]
    fn cache_control_type_serialize() {
        assert_eq!(
            serde_json::to_string(&CacheControlType::Ephemeral).unwrap(),
            "\"ephemeral\""
        );
    }

    #[test]
    fn cache_control_type_deserialize() {
        assert_eq!(
            serde_json::from_str::<CacheControlType>("\"ephemeral\"").unwrap(),
            CacheControlType::Ephemeral
        );
    }
}