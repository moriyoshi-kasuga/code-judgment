#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[repr(transparent)]
pub struct MsTime(u64);

impl MsTime {
    pub const fn new(seconds: u64, ms: u64) -> Self {
        Self(seconds * 1000 + ms)
    }

    pub const fn new_ms(ms: u64) -> Self {
        Self(ms)
    }

    pub const fn new_seconds(seconds: u64) -> Self {
        Self(seconds * 1000)
    }

    pub const fn as_ms(&self) -> u64 {
        self.0
    }

    pub const fn as_seconds(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}
