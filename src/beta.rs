use std::fmt::Display;

/// The beta feature.
///
/// See also [the API reference](https://docs.anthropic.com/claude/reference/versions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Beta {
    /// tools-2024-04-04
    Tools2024_04_04,
    /// extended-cache-ttl-2025-04-11
    ExtendedCacheTtl2025_04_11,
}

impl Default for Beta {
    fn default() -> Self {
        Self::Tools2024_04_04
    }
}

impl Display for Beta {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Beta::Tools2024_04_04 => {
                write!(f, "tools-2024-04-04")
            },
            | Beta::ExtendedCacheTtl2025_04_11 => {
                write!(f, "extended-cache-ttl-2025-04-11")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Beta::default(), Beta::Tools2024_04_04,);
    }

    #[test]
    fn display() {
        assert_eq!(
            Beta::Tools2024_04_04.to_string(),
            "tools-2024-04-04",
        );
        assert_eq!(
            Beta::ExtendedCacheTtl2025_04_11.to_string(),
            "extended-cache-ttl-2025-04-11",
        );
    }
}
