#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    more_convert::EnumRepr,
    more_convert::EnumArray,
    more_convert::VariantName,
)]
#[repr(u32)]
#[enum_repr(serde)]
#[variant_name(rename_all = "camelCase")]
pub enum Language {
    Rust1_82 = 1,
    Go1_23 = 2,
    Python3_13 = 3,
}
