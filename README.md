# nagara Network's Reference Smart Contract

This contract lives on [nagara Network - Self Identify](https://contracts-ui.nagara.network/contract/gr4By4jrrXW91XPdZhEMmT96vW3zgPyTzgTwtDhf1UXjeuoyC?rpc=wss://boot.nagara.network).

## Purpose

The purpose of this dApp is to demonstrate:

- CI/CD enabled smart contract deployment with nagara Network (upgradability)
- Execution RefTime & ProofSize probes/references
- Custom storage migrations (not-yet-implemented)
- Custom smart contract data type

## Functionalities

Current functionalities:

- Only authorities can upgrade the code on-chain and reset all data
- Only authorities can add/remove identity verifier
- Current identity (pseudonym) length must be between 4 to 32 chars (alphanumeric & underscore)
- No one can update one's identity except themself, although the verifier can update the verified status
- No one can unverify one identity except when they update it themself
