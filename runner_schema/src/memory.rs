use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryLimit(u64);

impl MemoryLimit {
    pub const fn new_bytes(limit: u64) -> Self {
        Self(limit)
    }

    pub const fn new_kilobytes(limit: u64) -> Self {
        Self(limit * 1024)
    }

    pub const fn new_megabytes(limit: u64) -> Self {
        Self(limit * 1024 * 1024)
    }

    pub const fn as_bytes(&self) -> u64 {
        self.0
    }

    pub const fn as_kilobytes(&self) -> u64 {
        self.0 / 1024
    }

    pub const fn as_megabytes(&self) -> u64 {
        self.0 / (1024 * 1024)
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum FromStrMemoryLimitError {
    #[error("invalid suffix: {0} is not a valid suffix")]
    InvalidSuffix(String),
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
    #[error("invalid string: {0} is too short. Must be at least 2 characters")]
    ShortLength(String),
}

impl FromStr for MemoryLimit {
    type Err = FromStrMemoryLimitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(FromStrMemoryLimitError::ShortLength(s.to_string()));
        }
        let (num_str, suffix) = s.split_at(s.len() - 1);
        let num = num_str.parse::<u64>()?;
        let limit = match suffix {
            "B" => MemoryLimit::new_bytes(num),
            "K" => MemoryLimit::new_kilobytes(num),
            "M" => MemoryLimit::new_megabytes(num),
            _ => {
                return Err(FromStrMemoryLimitError::InvalidSuffix(suffix.to_string()));
            }
        };
        Ok(limit)
    }
}

impl Display for MemoryLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.as_megabytes() > 0 {
            write!(f, "{}M", self.as_megabytes())
        } else if self.as_kilobytes() > 0 {
            write!(f, "{}K", self.as_kilobytes())
        } else {
            write!(f, "{}B", self.as_bytes())
        }
    }
}

impl<'de> serde::Deserialize<'de> for MemoryLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MemoryLimit::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for MemoryLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mega() {
        let limit = MemoryLimit::new_megabytes(10);
        assert_eq!(limit.as_megabytes(), 10);
        assert_eq!(limit.as_kilobytes(), 10 * 1024);
        assert_eq!(limit.as_bytes(), 10 * 1024 * 1024);
    }

    #[test]
    fn kilo() {
        let limit = MemoryLimit::new_kilobytes(10);
        assert_eq!(limit.as_megabytes(), 0);
        assert_eq!(limit.as_kilobytes(), 10);
        assert_eq!(limit.as_bytes(), 10 * 1024);
    }

    #[test]
    fn bytes() {
        let limit = MemoryLimit::new_bytes(10);
        assert_eq!(limit.as_megabytes(), 0);
        assert_eq!(limit.as_kilobytes(), 0);
        assert_eq!(limit.as_bytes(), 10);
    }

    #[test]
    fn from_str() {
        assert_eq!(MemoryLimit::from_str("10M").unwrap().as_megabytes(), 10);
        assert_eq!(MemoryLimit::from_str("10K").unwrap().as_kilobytes(), 10);
        assert_eq!(MemoryLimit::from_str("10B").unwrap().as_bytes(), 10);
        assert_eq!(
            MemoryLimit::from_str("10X").unwrap_err(),
            FromStrMemoryLimitError::InvalidSuffix("X".to_string())
        );
        assert_eq!(
            MemoryLimit::from_str("10").unwrap_err(),
            FromStrMemoryLimitError::InvalidSuffix("0".to_string())
        );
        assert_eq!(
            MemoryLimit::from_str("1").unwrap_err(),
            FromStrMemoryLimitError::ShortLength("1".to_string())
        );
        assert_eq!(
            MemoryLimit::from_str("").unwrap_err(),
            FromStrMemoryLimitError::ShortLength("".to_string())
        );
    }

    #[test]
    fn display() {
        assert_eq!(MemoryLimit::new_megabytes(10).to_string(), "10M");
        assert_eq!(MemoryLimit::new_kilobytes(10).to_string(), "10K");
        assert_eq!(MemoryLimit::new_bytes(10).to_string(), "10B");
    }

    #[test]
    fn deserialize() {
        let limit: MemoryLimit = serde_json::from_str(r#""10M""#).unwrap();
        assert_eq!(limit.as_megabytes(), 10);
        let limit: MemoryLimit = serde_json::from_str(r#""10K""#).unwrap();
        assert_eq!(limit.as_kilobytes(), 10);
        let limit: MemoryLimit = serde_json::from_str(r#""10B""#).unwrap();
        assert_eq!(limit.as_bytes(), 10);
    }

    #[test]
    fn serialize() {
        let limit = MemoryLimit::new_megabytes(10);
        let s = serde_json::to_string(&limit).unwrap();
        assert_eq!(s, r#""10M""#);
        let limit = MemoryLimit::new_kilobytes(10);
        let s = serde_json::to_string(&limit).unwrap();
        assert_eq!(s, r#""10K""#);
        let limit = MemoryLimit::new_bytes(10);
        let s = serde_json::to_string(&limit).unwrap();
        assert_eq!(s, r#""10B""#);
    }
}
