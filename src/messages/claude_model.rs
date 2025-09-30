use crate::macros::impl_enum_string_serialization;
use std::fmt::Display;

/// The model that will complete your prompt.
///
/// See [models](https://docs.anthropic.com/claude/docs/models-overview) for additional details and options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaudeModel {
    // Claude 3 Opus
    /// Claude 3 Opus at 2024/02/29.
    Claude3Opus20240229,
    // Claude 3 Sonnet
    /// Claude 3 Sonnet at 2024/02/29.
    Claude3Sonnet20240229,
    // Claude 3 Haiku
    /// Claude 3 Haiku at 2024/03/07.
    Claude3Haiku20240307,
    // Claude 3.5 Sonnet
    /// Claude 3.5 Sonnet at 2024/06/20
    Claude35Sonnet20240620,
    // Claude 3.5 Haiku
    /// Claude 3.5 Haiku at 2024/10/22
    Claude35Haiku20241022,
    // Claude 3.7 Sonnet
    /// Claude 3.7 Sonnet at 2024/06/20
    Claude37Sonnet20250219,
    // Claude 4.0 Opus
    Claude4Opus20250514,
    // Claude 4.0 Sonnet
    Claude4Sonnet20250514,
    // Claude 4.1 Opus
    Claude41Opus20250805,
    // Claude 4.1 Sonnet
    Claude41Sonnet20250805,
    // Claude 4.5 Sonnet
    Claude45Sonnet20250929,
}

impl Default for ClaudeModel {
    fn default() -> Self {
        Self::Claude3Sonnet20240229
    }
}

impl Display for ClaudeModel {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ClaudeModel::Claude3Opus20240229 => {
                write!(f, "claude-3-opus-20240229")
            },
            | ClaudeModel::Claude3Sonnet20240229 => {
                write!(f, "claude-3-sonnet-20240229")
            },
            | ClaudeModel::Claude3Haiku20240307 => {
                write!(f, "claude-3-haiku-20240307")
            },
            | ClaudeModel::Claude35Sonnet20240620 => {
                write!(f, "claude-3-5-sonnet-20240620")
            },
            | ClaudeModel::Claude35Haiku20241022 => {
                write!(f, "claude-3-5-haiku-20241022")
            },
            | ClaudeModel::Claude37Sonnet20250219 => {
                write!(f, "claude-3-7-sonnet-20250219")
            },
            | ClaudeModel::Claude4Opus20250514 => {
                write!(f, "claude-opus-4-20250514")
            },
            | ClaudeModel::Claude4Sonnet20250514 => {
                write!(f, "claude-sonnet-4-20250514")
            },
            | ClaudeModel::Claude41Opus20250805 => {
                write!(f, "claude-opus-4-1-20250805")
            },
            | ClaudeModel::Claude41Sonnet20250805 => {
                write!(f, "claude-sonnet-4-1-20250805")
            },
            | ClaudeModel::Claude45Sonnet20250929 => {
                write!(f, "claude-sonnet-4-5-20250929")
            },
        }
    }
}

impl ClaudeModel {
    pub(crate) fn max_tokens(&self) -> u32 {
        match self {
            | ClaudeModel::Claude3Opus20240229 => 4096,
            | ClaudeModel::Claude3Sonnet20240229 => 4096,
            | ClaudeModel::Claude3Haiku20240307 => 4096,
            | ClaudeModel::Claude35Sonnet20240620 => 4096,
            | ClaudeModel::Claude35Haiku20241022 => 8192,
            | ClaudeModel::Claude37Sonnet20250219 => 64000,
            | ClaudeModel::Claude4Opus20250514 => 32000,
            | ClaudeModel::Claude4Sonnet20250514 => 64000,
            | ClaudeModel::Claude41Opus20250805 => 32000,
            | ClaudeModel::Claude41Sonnet20250805 => 64000,
            | ClaudeModel::Claude45Sonnet20250929 => 64000,
        }
    }
}

impl_enum_string_serialization!(
    ClaudeModel,
    Claude3Opus20240229 => "claude-3-opus-20240229",
    Claude3Sonnet20240229 => "claude-3-sonnet-20240229",
    Claude3Haiku20240307 => "claude-3-haiku-20240307",
    Claude35Sonnet20240620 => "claude-3-5-sonnet-20240620",
    Claude35Haiku20241022 => "claude-3-5-haiku-20241022",
    Claude37Sonnet20250219 => "claude-3-7-sonnet-20250219",
    Claude4Opus20250514 => "claude-opus-4-20250514",
    Claude4Sonnet20250514 => "claude-sonnet-4-20250514",
    Claude41Opus20250805 => "claude-opus-4-1-20250805",
    Claude41Sonnet20250805 => "claude-sonnet-4-1-20250805",
    Claude45Sonnet20250929 => "claude-sonnet-4-5-20250929"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            ClaudeModel::default(),
            ClaudeModel::Claude3Sonnet20240229
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            ClaudeModel::Claude3Opus20240229.to_string(),
            "claude-3-opus-20240229"
        );
        assert_eq!(
            ClaudeModel::Claude3Sonnet20240229.to_string(),
            "claude-3-sonnet-20240229"
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku20240307.to_string(),
            "claude-3-haiku-20240307"
        );
        assert_eq!(
            ClaudeModel::Claude35Sonnet20240620.to_string(),
            "claude-3-5-sonnet-20240620"
        );
    }

    #[test]
    fn max_tokens() {
        assert_eq!(
            ClaudeModel::Claude3Opus20240229.max_tokens(),
            4096
        );
        assert_eq!(
            ClaudeModel::Claude3Sonnet20240229.max_tokens(),
            4096
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku20240307.max_tokens(),
            4096
        );
        assert_eq!(
            ClaudeModel::Claude35Sonnet20240620.max_tokens(),
            4096
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::from_str::<ClaudeModel>("\"claude-3-opus-20240229\"")
                .unwrap(),
            ClaudeModel::Claude3Opus20240229
        );
        assert_eq!(
            serde_json::from_str::<ClaudeModel>("\"claude-3-sonnet-20240229\"")
                .unwrap(),
            ClaudeModel::Claude3Sonnet20240229
        );
        assert_eq!(
            serde_json::from_str::<ClaudeModel>("\"claude-3-haiku-20240307\"")
                .unwrap(),
            ClaudeModel::Claude3Haiku20240307
        );
        assert_eq!(
            serde_json::from_str::<ClaudeModel>(
                "\"claude-3-5-sonnet-20240620\""
            )
            .unwrap(),
            ClaudeModel::Claude35Sonnet20240620
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::to_string(&ClaudeModel::Claude3Opus20240229).unwrap(),
            "\"claude-3-opus-20240229\""
        );
        assert_eq!(
            serde_json::to_string(&ClaudeModel::Claude3Sonnet20240229).unwrap(),
            "\"claude-3-sonnet-20240229\""
        );
        assert_eq!(
            serde_json::to_string(&ClaudeModel::Claude3Haiku20240307).unwrap(),
            "\"claude-3-haiku-20240307\""
        );
        assert_eq!(
            serde_json::to_string(&ClaudeModel::Claude35Sonnet20240620)
                .unwrap(),
            "\"claude-3-5-sonnet-20240620\""
        );
    }
}
