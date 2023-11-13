pub type Result<T> = core::result::Result<T, Error>;

#[repr(u32)]
#[derive(
    thiserror_no_std::Error, Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    #[error("Only alphanumeric and underscore characters allowed")]
    BadStringInput,
    #[error("Pseudonym length must be between 4 to 64 characters")]
    BadPseudonymLength,
    #[error("Pseudonym already taken")]
    TakenPseudonym,
    #[error("Insufficient permission")]
    InsufficientPermission,
    #[error("Verifier already exist")]
    VerifierAlreadyExist,
    #[error("Verifier is non-existence")]
    VerifierNotExist,
    #[error("Pseudonym is non-existence")]
    PseudonymNotExist,
    #[error("Pseudonym Already verified")]
    PseudonymAlreadyVerified,
}
