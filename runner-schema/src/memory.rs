use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Memory(u64);

impl Memory {
    pub const fn new_bytes(memory: u64) -> Self {
        Self(memory)
    }

    pub const fn new_kilobytes(memory: u64) -> Self {
        Self(memory * 1024)
    }

    pub const fn new_megabytes(memory: u64) -> Self {
        Self(memory * 1024 * 1024)
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
pub enum FromStrMemoryError {
    #[error("invalid suffix: {0} is not a valid suffix")]
    InvalidSuffix(String),
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
    #[error("invalid string: {0} is too short. Must be at least 2 characters")]
    ShortLength(String),
}

impl FromStr for Memory {
    type Err = FromStrMemoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(FromStrMemoryError::ShortLength(s.to_string()));
        }
        let (num_str, suffix) = s.split_at(s.len() - 1);
        let num = num_str.parse::<u64>()?;
        let memory = match suffix {
            "B" => Memory::new_bytes(num),
            "K" => Memory::new_kilobytes(num),
            "M" => Memory::new_megabytes(num),
            _ => {
                return Err(FromStrMemoryError::InvalidSuffix(suffix.to_string()));
            }
        };
        Ok(memory)
    }
}

impl Display for Memory {
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

impl<'de> serde::Deserialize<'de> for Memory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Memory::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Memory {
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
        let memory = Memory::new_megabytes(10);
        assert_eq!(memory.as_megabytes(), 10);
        assert_eq!(memory.as_kilobytes(), 10 * 1024);
        assert_eq!(memory.as_bytes(), 10 * 1024 * 1024);
    }

    #[test]
    fn kilo() {
        let memory = Memory::new_kilobytes(10);
        assert_eq!(memory.as_megabytes(), 0);
        assert_eq!(memory.as_kilobytes(), 10);
        assert_eq!(memory.as_bytes(), 10 * 1024);
    }

    #[test]
    fn bytes() {
        let memory = Memory::new_bytes(10);
        assert_eq!(memory.as_megabytes(), 0);
        assert_eq!(memory.as_kilobytes(), 0);
        assert_eq!(memory.as_bytes(), 10);
    }

    #[test]
    fn from_str() {
        assert_eq!(Memory::from_str("10M").unwrap().as_megabytes(), 10);
        assert_eq!(Memory::from_str("10K").unwrap().as_kilobytes(), 10);
        assert_eq!(Memory::from_str("10B").unwrap().as_bytes(), 10);
        assert_eq!(
            Memory::from_str("10X").unwrap_err(),
            FromStrMemoryError::InvalidSuffix("X".to_string())
        );
        assert_eq!(
            Memory::from_str("10").unwrap_err(),
            FromStrMemoryError::InvalidSuffix("0".to_string())
        );
        assert_eq!(
            Memory::from_str("1").unwrap_err(),
            FromStrMemoryError::ShortLength("1".to_string())
        );
        assert_eq!(
            Memory::from_str("").unwrap_err(),
            FromStrMemoryError::ShortLength("".to_string())
        );
    }

    #[test]
    fn display() {
        assert_eq!(Memory::new_megabytes(10).to_string(), "10M");
        assert_eq!(Memory::new_kilobytes(10).to_string(), "10K");
        assert_eq!(Memory::new_bytes(10).to_string(), "10B");
    }

    #[test]
    fn deserialize() {
        let memory: Memory = serde_json::from_str(r#""10M""#).unwrap();
        assert_eq!(memory.as_megabytes(), 10);
        let memory: Memory = serde_json::from_str(r#""10K""#).unwrap();
        assert_eq!(memory.as_kilobytes(), 10);
        let memory: Memory = serde_json::from_str(r#""10B""#).unwrap();
        assert_eq!(memory.as_bytes(), 10);
    }

    #[test]
    fn serialize() {
        let memory = Memory::new_megabytes(10);
        let s = serde_json::to_string(&memory).unwrap();
        assert_eq!(s, r#""10M""#);
        let memory = Memory::new_kilobytes(10);
        let s = serde_json::to_string(&memory).unwrap();
        assert_eq!(s, r#""10K""#);
        let memory = Memory::new_bytes(10);
        let s = serde_json::to_string(&memory).unwrap();
        assert_eq!(s, r#""10B""#);
    }
}
