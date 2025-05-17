#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[repr(u32)]
pub enum Language {
    Rust1_86,
    Go1_24,
    Python3_13,
}
