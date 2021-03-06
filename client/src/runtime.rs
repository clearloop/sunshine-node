use crate::frame::shares_atomic::SharesAtomic;
use codec::{Decode, Encode};
use sp_runtime::{
    generic::{Era, Header},
    traits::{BlakeTwo256, IdentifyAccount, SignedExtension, Verify},
    transaction_validity::TransactionValidityError,
    MultiSignature, OpaqueExtrinsic,
};
use std::marker::PhantomData;
use substrate_subxt::{
    balances::{AccountData, Balances},
    system::System,
    CheckEra, CheckGenesis, CheckNonce, CheckVersion, CheckWeight, SignedExtra,
};

/// Concrete type definitions compatible w/ sunshine's runtime aka `suntime`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Runtime;

impl System for Runtime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = pallet_indices::address::Address<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for Runtime {
    type Balance = u128;
}

impl SharesAtomic for Runtime {
    type OrgId = u64;
    type ShareId = u64;
}

pub type RuntimeExtra = Extra<Runtime>;

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct Extra<T: System> {
    version: u32,
    nonce: T::Index,
    genesis_hash: T::Hash,
}

impl<T: System + Balances + Send + Sync> SignedExtra<T> for Extra<T> {
    type Extra = (
        CheckVersion<T>,
        CheckGenesis<T>,
        CheckEra<T>,
        CheckNonce<T>,
        CheckWeight<T>,
    );

    fn new(version: u32, nonce: T::Index, genesis_hash: T::Hash) -> Self {
        Self {
            version,
            nonce,
            genesis_hash,
        }
    }

    fn extra(&self) -> Self::Extra {
        (
            CheckVersion(PhantomData, self.version),
            CheckGenesis(PhantomData, self.genesis_hash),
            CheckEra((Era::Immortal, PhantomData), self.genesis_hash),
            CheckNonce(self.nonce),
            CheckWeight(PhantomData),
        )
    }
}

impl<T: System + Balances + Send + Sync> SignedExtension for Extra<T> {
    const IDENTIFIER: &'static str = "DefaultExtra";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = <<Self as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned;
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        self.extra().additional_signed()
    }
}
