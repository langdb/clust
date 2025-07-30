use std::fmt::Display;

use crate::macros::impl_display_for_serialize;
use crate::messages::{ContentBlock, TextContentBlock};

/// Cache control for system prompt content blocks.
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

/// System prompt content block with cache control.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SystemPromptContentBlock {
    /// The content block.
    #[serde(flatten)]
    pub content: ContentBlock,
    /// Optional cache control for this content block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

impl SystemPromptContentBlock {
    /// Creates a new system prompt content block from a text content block.
    pub fn text(text: &str) -> Self {
        Self {
            content: ContentBlock::Text(TextContentBlock::new(text)),
            cache_control: None,
        }
    }

    /// Creates a new system prompt content block from a text content block with cache control.
    pub fn text_with_cache_control(text: &str, cache_control: CacheControl) -> Self {
        Self {
            content: ContentBlock::Text(TextContentBlock::new(text)),
            cache_control: Some(cache_control),
        }
    }

    /// Creates a new system prompt content block from any content block.
    pub fn from_content_block(content: ContentBlock) -> Self {
        Self {
            content,
            cache_control: None,
        }
    }

    /// Creates a new system prompt content block from any content block with cache control.
    pub fn from_content_block_with_cache_control(content: ContentBlock, cache_control: CacheControl) -> Self {
        Self {
            content,
            cache_control: Some(cache_control),
        }
    }
}

/// System prompt.
///
/// A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role.
/// See our [guide to system prompts](https://docs.anthropic.com/claude/docs/system-prompts).
///
/// This can be either a simple string or an array of content blocks with cache control.
#[derive(Debug, Clone, PartialEq)]
pub enum SystemPrompt {
    /// Simple string system prompt (legacy format).
    Simple(String),
    /// Advanced system prompt with content blocks and cache control.
    Advanced(Vec<SystemPromptContentBlock>),
}

impl Default for SystemPrompt {
    fn default() -> Self {
        Self::Simple(String::new())
    }
}

impl Display for SystemPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemPrompt::Simple(text) => write!(f, "{}", text),
            SystemPrompt::Advanced(blocks) => {
                for (i, block) in blocks.iter().enumerate() {
                    if i > 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}", block.content)?;
                }
                Ok(())
            }
        }
    }
}

impl From<String> for SystemPrompt {
    fn from(value: String) -> Self {
        Self::Simple(value)
    }
}

impl From<&str> for SystemPrompt {
    fn from(value: &str) -> Self {
        Self::Simple(value.to_string())
    }
}

impl From<Vec<SystemPromptContentBlock>> for SystemPrompt {
    fn from(blocks: Vec<SystemPromptContentBlock>) -> Self {
        Self::Advanced(blocks)
    }
}

impl SystemPrompt {
    /// Creates a new simple system prompt.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::Simple(value.into())
    }

    /// Creates a new advanced system prompt with content blocks.
    pub fn from_content_blocks(blocks: Vec<SystemPromptContentBlock>) -> Self {
        Self::Advanced(blocks)
    }

    /// Creates a new advanced system prompt from text blocks.
    pub fn from_text_blocks(texts: Vec<&str>) -> Self {
        let blocks: Vec<SystemPromptContentBlock> = texts
            .iter()
            .map(|text| SystemPromptContentBlock::text(text))
            .collect();
        Self::Advanced(blocks)
    }

    /// Creates a new advanced system prompt from text blocks with cache control.
    pub fn from_text_blocks_with_cache_control(
        texts_with_cache: Vec<(&str, Option<CacheControl>)>,
    ) -> Self {
        let blocks: Vec<SystemPromptContentBlock> = texts_with_cache
            .iter()
            .map(|(text, cache_control)| {
                if let Some(cache_control) = cache_control {
                    SystemPromptContentBlock::text_with_cache_control(text, cache_control.clone())
                } else {
                    SystemPromptContentBlock::text(text)
                }
            })
            .collect();
        Self::Advanced(blocks)
    }
}

// Custom serialization for SystemPrompt
impl serde::Serialize for SystemPrompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SystemPrompt::Simple(text) => text.serialize(serializer),
            SystemPrompt::Advanced(blocks) => {
                let mut block_map = serde_json::Map::new();
                for (i, block) in blocks.iter().enumerate() {
                    let mut block_value = serde_json::to_value(&block.content)
                        .map_err(serde::ser::Error::custom)?;
                    
                    if let Some(ref cache_control) = block.cache_control {
                        if let serde_json::Value::Object(ref mut obj) = block_value {
                            obj.insert(
                                "cache_control".to_string(),
                                serde_json::to_value(cache_control)
                                    .map_err(serde::ser::Error::custom)?,
                            );
                        }
                    }
                    
                    block_map.insert(i.to_string(), block_value);
                }
                serde_json::Value::Array(
                    blocks
                        .iter()
                        .map(|block| {
                            let mut block_value = serde_json::to_value(&block.content)
                                .map_err(serde::ser::Error::custom)?;
                            
                            if let Some(ref cache_control) = block.cache_control {
                                if let serde_json::Value::Object(ref mut obj) = block_value {
                                    obj.insert(
                                        "cache_control".to_string(),
                                        serde_json::to_value(cache_control)
                                            .map_err(serde::ser::Error::custom)?,
                                    );
                                }
                            }
                            
                            Ok(block_value)
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
                .serialize(serializer)
            }
        }
    }
}

// Custom deserialization for SystemPrompt
impl<'de> serde::Deserialize<'de> for SystemPrompt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        
        match value {
            serde_json::Value::String(text) => Ok(SystemPrompt::Simple(text)),
            serde_json::Value::Array(array) => {
                let blocks: Result<Vec<SystemPromptContentBlock>, _> = array
                    .iter()
                    .map(|item| {
                        if let serde_json::Value::Object(obj) = item {
                            let mut block_obj = obj.clone();
                            let cache_control = block_obj.remove("cache_control")
                                .map(|v| serde_json::from_value(v))
                                .transpose()
                                .map_err(serde::de::Error::custom)?;
                            
                            let content = serde_json::from_value(serde_json::Value::Object(block_obj))
                                .map_err(serde::de::Error::custom)?;
                            
                            Ok(SystemPromptContentBlock {
                                content,
                                cache_control,
                            })
                        } else {
                            let content = serde_json::from_value(item.clone())
                                .map_err(serde::de::Error::custom)?;
                            Ok(SystemPromptContentBlock {
                                content,
                                cache_control: None,
                            })
                        }
                    })
                    .collect();
                
                Ok(SystemPrompt::Advanced(blocks?))
            }
            _ => Err(serde::de::Error::custom("expected string or array")),
        }
    }
}

impl_display_for_serialize!(SystemPrompt);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{ContentBlock, TextContentBlock};

    #[test]
    fn new() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(system_prompt, SystemPrompt::Simple("system-prompt".to_string()));
    }

    #[test]
    fn default() {
        assert_eq!(SystemPrompt::default(), SystemPrompt::Simple("".to_string()));
    }

    #[test]
    fn display_simple() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(system_prompt.to_string(), "system-prompt");
    }

    #[test]
    fn display_advanced() {
        let blocks = vec![
            SystemPromptContentBlock::text("First block"),
            SystemPromptContentBlock::text("Second block"),
        ];
        let system_prompt = SystemPrompt::Advanced(blocks);
        assert_eq!(system_prompt.to_string(), "First block\nSecond block");
    }

    #[test]
    fn serialize_simple() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            serde_json::to_string(&system_prompt).unwrap(),
            "\"system-prompt\""
        );
    }

    #[test]
    fn serialize_advanced() {
        let blocks = vec![
            SystemPromptContentBlock::text("First block"),
            SystemPromptContentBlock::text_with_cache_control(
                "Second block",
                CacheControl::default(),
            ),
        ];
        let system_prompt = SystemPrompt::Advanced(blocks);
        let serialized = serde_json::to_string(&system_prompt).unwrap();
        assert!(serialized.contains("First block"));
        assert!(serialized.contains("Second block"));
        assert!(serialized.contains("cache_control"));
    }

    #[test]
    fn deserialize_simple() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            serde_json::from_str::<SystemPrompt>("\"system-prompt\"").unwrap(),
            system_prompt
        );
    }

    #[test]
    fn deserialize_advanced() {
        let json = r#"[
            {"type": "text", "text": "First block"},
            {"type": "text", "text": "Second block", "cache_control": {"type": "ephemeral"}}
        ]"#;
        let system_prompt = serde_json::from_str::<SystemPrompt>(json).unwrap();
        match system_prompt {
            SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                assert_eq!(blocks[0].cache_control, None);
                assert!(blocks[1].cache_control.is_some());
            }
            _ => panic!("Expected advanced system prompt"),
        }
    }

    #[test]
    fn from_text_blocks() {
        let texts = vec!["First block", "Second block"];
        let system_prompt = SystemPrompt::from_text_blocks(texts);
        match system_prompt {
            SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                assert_eq!(blocks[0].cache_control, None);
                assert_eq!(blocks[1].cache_control, None);
            }
            _ => panic!("Expected advanced system prompt"),
        }
    }

    #[test]
    fn from_text_blocks_with_cache_control() {
        let texts_with_cache = vec![
            ("First block", None),
            ("Second block", Some(CacheControl::default())),
        ];
        let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(texts_with_cache);
        match system_prompt {
            SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                assert_eq!(blocks[0].cache_control, None);
                assert!(blocks[1].cache_control.is_some());
            }
            _ => panic!("Expected advanced system prompt"),
        }
    }

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
}
