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

    pub const fn as_seconds_ceil(&self) -> u64 {
        self.0.div_ceil(1000)
    }

    pub const fn add_seconds(&self, seconds: u64) -> Self {
        Self(self.0 + seconds * 1000)
    }

    pub const fn add_ms(&self, ms: u64) -> Self {
        Self(self.0 + ms)
    }

    pub fn from_str_mm_ss_ms(s: &str) -> Option<Self> {
        let mut parts = s.split(':');
        let minutes = parts.next()?.parse::<u64>().ok()?;
        let ss_ms = parts.next()?.split('.').collect::<Vec<_>>();
        if ss_ms.len() != 2 {
            return None;
        }
        let seconds = ss_ms[0].parse::<u64>().ok()?;
        let ms = ss_ms[1].parse::<u64>().ok()?;
        if ms > 999 {
            return None;
        }
        let total_seconds = minutes * 60 + seconds;
        Some(Self::new(total_seconds, ms))
    }
}
