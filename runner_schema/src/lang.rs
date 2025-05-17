#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    more_convert::EnumRepr,
)]
#[repr(u32)]
pub enum Language {
    Rust1_82 = 1,
    Go1_23 = 2,
    Python3_13 = 3,
}

impl Language {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}
