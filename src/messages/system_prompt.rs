use std::fmt::Display;

use crate::messages::{CacheControl, ContentBlock, TextContentBlock};

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
    Advanced(Vec<ContentBlock>),
}

impl Default for SystemPrompt {
    fn default() -> Self {
        Self::Simple(String::new())
    }
}

impl Display for SystemPrompt {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | SystemPrompt::Simple(text) => write!(f, "{}", text),
            | SystemPrompt::Advanced(blocks) => {
                for (i, block) in blocks.iter().enumerate() {
                    if i > 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}", block)?;
                }
                Ok(())
            },
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

impl From<Vec<ContentBlock>> for SystemPrompt {
    fn from(blocks: Vec<ContentBlock>) -> Self {
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
    pub fn from_content_blocks(blocks: Vec<ContentBlock>) -> Self {
        Self::Advanced(blocks)
    }

    /// Creates a new advanced system prompt from text blocks.
    pub fn from_text_blocks(texts: Vec<&str>) -> Self {
        let blocks: Vec<ContentBlock> = texts
            .iter()
            .map(|text| {
                ContentBlock::Text(TextContentBlock::new(text.to_string()))
            })
            .collect();
        Self::Advanced(blocks)
    }

    /// Creates a new advanced system prompt from text blocks with cache control.
    pub fn from_text_blocks_with_cache_control(
        texts_with_cache: Vec<(&str, Option<CacheControl>)>
    ) -> Self {
        let blocks: Vec<ContentBlock> = texts_with_cache
            .iter()
            .map(|(text, cache_control)| {
                if let Some(cache_control) = cache_control {
                    ContentBlock::Text(
                        TextContentBlock::new_with_cache_control(
                            text.to_string(),
                            cache_control.clone(),
                        ),
                    )
                } else {
                    ContentBlock::Text(TextContentBlock::new(text.to_string()))
                }
            })
            .collect();
        Self::Advanced(blocks)
    }
}

// Custom serialization for SystemPrompt
impl serde::Serialize for SystemPrompt {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            | SystemPrompt::Simple(text) => text.serialize(serializer),
            | SystemPrompt::Advanced(blocks) => blocks.serialize(serializer),
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
            | serde_json::Value::String(text) => Ok(SystemPrompt::Simple(text)),
            | serde_json::Value::Array(_) => {
                let blocks = Vec::<ContentBlock>::deserialize(
                    serde_json::to_value(value)
                        .map_err(serde::de::Error::custom)?,
                )
                .map_err(serde::de::Error::custom)?;
                Ok(SystemPrompt::Advanced(blocks))
            },
            | _ => Err(serde::de::Error::custom(
                "expected string or array",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{ContentBlock, TextContentBlock};

    #[test]
    fn new() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            system_prompt,
            SystemPrompt::Simple("system-prompt".to_string())
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            SystemPrompt::default(),
            SystemPrompt::Simple("".to_string())
        );
    }

    #[test]
    fn display_simple() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            system_prompt.to_string(),
            "system-prompt"
        );
    }

    #[test]
    fn display_advanced() {
        let blocks = vec![
            ContentBlock::Text(TextContentBlock::new("First block")),
            ContentBlock::Text(TextContentBlock::new("Second block")),
        ];
        let system_prompt = SystemPrompt::Advanced(blocks);
        assert_eq!(
            system_prompt.to_string(),
            "First block\nSecond block"
        );
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
            ContentBlock::Text(TextContentBlock::new("First block")),
            ContentBlock::Text(
                TextContentBlock::new_with_cache_control(
                    "Second block",
                    CacheControl::default(),
                ),
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
            | SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                // First block should have no cache control
                if let ContentBlock::Text(text_block) = &blocks[0] {
                    assert_eq!(text_block.cache_control, None);
                } else {
                    panic!("Expected text block");
                }
                // Second block should have cache control
                if let ContentBlock::Text(text_block) = &blocks[1] {
                    assert!(
                        text_block
                            .cache_control
                            .is_some()
                    );
                } else {
                    panic!("Expected text block");
                }
            },
            | _ => panic!("Expected advanced system prompt"),
        }
    }

    #[test]
    fn from_text_blocks() {
        let texts = vec![
            "First block",
            "Second block",
        ];
        let system_prompt = SystemPrompt::from_text_blocks(texts);
        match system_prompt {
            | SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                if let ContentBlock::Text(text_block) = &blocks[0] {
                    assert_eq!(text_block.cache_control, None);
                }
                if let ContentBlock::Text(text_block) = &blocks[1] {
                    assert_eq!(text_block.cache_control, None);
                }
            },
            | _ => panic!("Expected advanced system prompt"),
        }
    }

    #[test]
    fn from_text_blocks_with_cache_control() {
        let texts_with_cache = vec![
            ("First block", None),
            (
                "Second block",
                Some(CacheControl::default()),
            ),
        ];
        let system_prompt =
            SystemPrompt::from_text_blocks_with_cache_control(texts_with_cache);
        match system_prompt {
            | SystemPrompt::Advanced(blocks) => {
                assert_eq!(blocks.len(), 2);
                if let ContentBlock::Text(text_block) = &blocks[0] {
                    assert_eq!(text_block.cache_control, None);
                }
                if let ContentBlock::Text(text_block) = &blocks[1] {
                    assert!(
                        text_block
                            .cache_control
                            .is_some()
                    );
                }
            },
            | _ => panic!("Expected advanced system prompt"),
        }
    }
}
