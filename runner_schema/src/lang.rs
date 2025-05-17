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
    Rust1_86 = 1,
    Go1_24 = 2,
    Python3_13 = 3,
}

impl Language {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}
