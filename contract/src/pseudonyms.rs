#[derive(
    core::clone::Clone,
    core::cmp::Eq,
    core::cmp::Ord,
    core::cmp::PartialEq,
    core::cmp::PartialOrd,
    core::fmt::Debug,
    core::hash::Hash,
    core::marker::Copy,
    scale::Decode,
    scale::Encode,
)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Identifier([u8; Self::MAX_LEN]);

impl Identifier {
    pub const MAX_LEN: usize = 32;
    pub const MIN_LEN: usize = 4;
    pub const RANGE_LEN: core::ops::RangeInclusive<usize> = Self::MIN_LEN..=Self::MAX_LEN;

    pub fn try_from_str(input: &str) -> crate::result::Result<Self> {
        if !Self::RANGE_LEN.contains(&input.len()) {
            return crate::result::Result::Err(crate::result::Error::BadPseudonymLength);
        }

        if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return crate::result::Result::Err(crate::result::Error::BadStringInput);
        }

        let mut inner = [0u8; Self::MAX_LEN];
        let inner_ref_mut = &mut inner[0..input.len()];
        inner_ref_mut.copy_from_slice(input.as_bytes());

        crate::result::Result::Ok(Self(inner))
    }
}

impl core::fmt::Display for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", core::ops::Deref::deref(self))
    }
}

impl core::ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        core::str::from_utf8(&self.0).expect("Should be infallible!")
    }
}

#[derive(
    core::clone::Clone,
    core::cmp::Eq,
    core::cmp::Ord,
    core::cmp::PartialEq,
    core::cmp::PartialOrd,
    core::fmt::Debug,
    core::hash::Hash,
    core::marker::Copy,
    scale::Decode,
    scale::Encode,
)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Info {
    pub owner: ink::primitives::AccountId,
    pub verified_by: core::option::Option<ink::primitives::AccountId>,
    pub verified_at: core::option::Option<u32>, // Blocknumber is always u32
}
