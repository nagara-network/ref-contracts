#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod pseudonyms;
pub mod result;

#[ink::contract]
mod self_identify {
    #[ink(storage)]
    pub struct SelfIdentify {
        authority: ink::primitives::AccountId,
        verifiers: ink::storage::Mapping<ink::primitives::AccountId, ()>,
        pseudonyms: ink::storage::Mapping<crate::pseudonyms::Identifier, crate::pseudonyms::Info>,
        accounts: ink::storage::Mapping<ink::primitives::AccountId, crate::pseudonyms::Identifier>,
    }

    #[ink(event)]
    pub struct IdentityVerified {
        verifier: ink::primitives::AccountId,
        pseudonym: ink::prelude::string::String,
        account: ink::primitives::AccountId,
        at: u32, // Blocknumber is always u32
    }

    #[ink(event)]
    pub struct IdentityInserted {
        account: ink::primitives::AccountId,
        pseudonym: ink::prelude::string::String,
        at: u32, // Blocknumber is always u32
    }

    #[ink(event)]
    pub struct IdentityRemoved {
        account: ink::primitives::AccountId,
        pseudonym: ink::prelude::string::String,
        at: u32, // Blocknumber is always u32
    }

    #[ink(event)]
    pub struct VerifierUpdated {
        who: ink::primitives::AccountId,
        removed: bool,
        at: u32, // Blocknumber is always u32
    }

    #[ink(event)]
    pub struct StoragePurged {
        at: u32, // Blocknumber is always u32
    }

    #[ink(event)]
    pub struct ContractUpgraded {
        new_code_hash: [u8; 32],
        at: u32, // Blocknumber is always u32
    }

    impl SelfIdentify {
        #[ink(constructor)]
        pub fn default() -> Self {
            let authority = Self::env().caller();
            let verifiers = Default::default();
            let pseudonyms = Default::default();
            let accounts = Default::default();

            Self {
                authority,
                verifiers,
                pseudonyms,
                accounts,
            }
        }

        #[ink(message)]
        pub fn authority_redirect_code(
            &mut self,
            new_code_hash: [u8; 32],
        ) -> crate::result::Result<()> {
            self.ensure_caller_is_authority()?;

            ink::env::set_code_hash(&new_code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {new_code_hash:?} due to {err:?}")
            });

            self.env().emit_event(ContractUpgraded {
                new_code_hash,
                at: self.env().block_number(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn authority_reset_all(&mut self) -> crate::result::Result<()> {
            self.ensure_caller_is_authority()?;
            self.verifiers = Default::default();

            self.env().emit_event(StoragePurged {
                at: self.env().block_number(),
            });

            crate::result::Result::Ok(())
        }

        #[ink(message)]
        pub fn authority_verifier(
            &mut self,
            verifier: ink::primitives::AccountId,
            add: bool,
        ) -> crate::result::Result<()> {
            self.ensure_caller_is_authority()?;

            let verifier_exist = self.verifiers.contains(verifier);

            if add {
                if verifier_exist {
                    return crate::result::Result::Err(crate::result::Error::VerifierAlreadyExist);
                }

                self.verifiers.insert(verifier, &());
            } else {
                if !verifier_exist {
                    return crate::result::Result::Err(crate::result::Error::VerifierNotExist);
                }

                self.verifiers.remove(verifier);
            }

            self.env().emit_event(VerifierUpdated {
                who: verifier,
                removed: !add,
                at: self.env().block_number(),
            });

            crate::result::Result::Ok(())
        }

        #[ink(message)]
        pub fn verifier_pseudonym_verify(
            &mut self,
            pseudonym: ink::prelude::string::String,
        ) -> crate::result::Result<()> {
            self.ensure_caller_is_verifier()?;
            let pseudonym = crate::pseudonyms::Identifier::try_from_str(&pseudonym)?;
            let mut info = self
                .pseudonyms
                .get(pseudonym)
                .ok_or(crate::result::Error::PseudonymNotExist)?;

            if info.verified_at.is_some() {
                return crate::result::Result::Err(crate::result::Error::PseudonymAlreadyVerified);
            }

            self.pseudonyms.remove(pseudonym);
            info.verified_at = Some(self.env().block_number());
            info.verified_by = Some(self.caller());
            self.pseudonyms.insert(pseudonym, &info);
            let account = info.owner;

            self.env().emit_event(IdentityVerified {
                verifier: self.caller(),
                pseudonym: core::ops::Deref::deref(&pseudonym).into(),
                account,
                at: self.env().block_number(),
            });

            crate::result::Result::Ok(())
        }

        #[ink(message)]
        pub fn any_add_or_update_pseudonym(
            &mut self,
            pseudonym: ink::prelude::string::String,
        ) -> crate::result::Result<()> {
            let pseudonym = crate::pseudonyms::Identifier::try_from_str(&pseudonym)?;
            let info = crate::pseudonyms::Info {
                owner: self.caller(),
                verified_by: None,
                verified_at: None,
            };

            if self.accounts.contains(self.caller()) {
                let old_pseudonym = self.accounts.get(self.caller()).unwrap();
                self.accounts.remove(self.caller());
                self.pseudonyms.remove(old_pseudonym);

                self.env().emit_event(IdentityRemoved {
                    account: self.caller(),
                    pseudonym: core::ops::Deref::deref(&old_pseudonym).into(),
                    at: self.env().block_number(),
                })
            }

            self.accounts.insert(self.caller(), &pseudonym);
            self.pseudonyms.insert(pseudonym, &info);

            self.env().emit_event(IdentityInserted {
                account: self.caller(),
                pseudonym: core::ops::Deref::deref(&pseudonym).into(),
                at: self.env().block_number(),
            });

            crate::result::Result::Ok(())
        }

        #[ink(message)]
        pub fn any_get_pseudonym_of(
            &self,
            of: ink::primitives::AccountId,
        ) -> core::option::Option<ink::prelude::string::String> {
            self.accounts
                .get(of)
                .map(|x| core::ops::Deref::deref(&x).into())
        }

        #[ink(message)]
        pub fn any_get_pseudonym(&self) -> core::option::Option<ink::prelude::string::String> {
            self.accounts
                .get(self.caller())
                .map(|x| core::ops::Deref::deref(&x).into())
        }

        #[inline(always)]
        fn caller(&self) -> ink::primitives::AccountId {
            self.env().caller()
        }

        #[inline(always)]
        fn ensure_caller_is_authority(&self) -> crate::result::Result<()> {
            if self.caller() != self.authority {
                return crate::result::Result::Err(crate::result::Error::InsufficientPermission);
            }

            crate::result::Result::Ok(())
        }

        #[inline(always)]
        fn ensure_caller_is_verifier(&self) -> crate::result::Result<()> {
            if self.verifiers.contains(self.caller()) {
                return crate::result::Result::Err(crate::result::Error::VerifierNotExist);
            }

            crate::result::Result::Ok(())
        }
    }
}
