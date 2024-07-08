//! Structures, traits and impls related to `Paramater`s.
#[cfg(not(feature = "std"))]
use alloc::{collections::btree_map, format, string::String, vec::Vec};
use core::{num::NonZeroU64, time::Duration};
#[cfg(feature = "std")]
use std::collections::btree_map;

use iroha_data_model_derive::model;
use iroha_primitives::json::JsonString;
use nonzero_ext::nonzero;

pub use self::model::*;
use crate::name::Name;

/// Collection of [`CustomParameter`]s
pub(crate) type CustomParameters = btree_map::BTreeMap<CustomParameterId, CustomParameter>;

#[model]
mod model {
    use derive_more::{Constructor, Display, FromStr};
    use getset::{CopyGetters, Getters};
    use iroha_data_model_derive::IdEqOrdHash;
    use iroha_schema::IntoSchema;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use strum::EnumDiscriminants;

    use super::*;

    /// Id of a custom parameter
    #[derive(
        Debug,
        Display,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        FromStr,
        Constructor,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[ffi_type]
    pub struct CustomParameterId(pub Name);

    /// Limits that govern consensus operation
    #[derive(
        Debug,
        Display,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[display(fmt = "{block_time_ms},{commit_time_ms}_SL")]
    pub struct SumeragiParameters {
        /// Maximal amount of time (in milliseconds) a peer will wait before forcing creation of a new block.
        ///
        /// A block is created if this limit or [`BlockParameters::max_transactions`] limit is reached,
        /// whichever comes first. Regardless of the limits, an empty block is never created.
        pub block_time_ms: u64,
        /// Time (in milliseconds) a peer will wait for a block to be committed.
        ///
        /// If this period expires the block will request a view change
        pub commit_time_ms: u64,
    }

    /// Single Sumeragi parameter
    ///
    /// Check [`SumeragiParameters`] for more details
    #[derive(
        Debug,
        Display,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Decode,
        Encode,
        Serialize,
        Deserialize,
        IntoSchema,
    )]
    pub enum SumeragiParameter {
        BlockTimeMs(u64),
        CommitTimeMs(u64),
    }

    /// Limits that a block must obey to be accepted.
    #[derive(
        Debug,
        Display,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        CopyGetters,
        Encode,
        Serialize,
        IntoSchema,
    )]
    #[display(fmt = "{max_transactions}_BL")]
    #[getset(get_copy = "pub")]
    pub struct BlockParameters {
        /// Maximal number of transactions in a block.
        ///
        /// A block is created if this limit is reached or [`SumeragiParameters::block_time_ms`] has expired,
        /// whichever comes first. Regardless of the limits, an empty block is never created.
        pub max_transactions: NonZeroU64,
    }

    /// Single block parameter
    ///
    /// Check [`BlockParameters`] for more details
    #[derive(
        Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Serialize, IntoSchema,
    )]
    pub enum BlockParameter {
        MaxTransactions(NonZeroU64),
    }

    /// Limits that a transaction must obey to be accepted.
    #[derive(
        Debug,
        Display,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        CopyGetters,
        Encode,
        Serialize,
        IntoSchema,
    )]
    #[display(fmt = "{max_instructions},{smart_contract_size}_TL")]
    #[getset(get_copy = "pub")]
    pub struct TransactionParameters {
        /// Maximum number of instructions per transaction
        pub max_instructions: NonZeroU64,
        /// Maximum size of wasm binary in bytes
        pub smart_contract_size: NonZeroU64,
    }

    /// Single transaction parameter
    ///
    /// Check [`TransactionParameters`] for more details
    #[derive(
        Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Serialize, IntoSchema,
    )]
    pub enum TransactionParameter {
        MaxInstructions(NonZeroU64),
        SmartContractSize(NonZeroU64),
    }

    /// Limits that a smart contract must obey at runtime to considered valid.
    #[derive(
        Debug,
        Display,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        CopyGetters,
        Encode,
        Serialize,
        IntoSchema,
    )]
    #[display(fmt = "{fuel},{memory}_SCL")]
    #[getset(get_copy = "pub")]
    pub struct SmartContractParameters {
        /// Maximum amount of fuel that a smart contract can consume
        pub fuel: NonZeroU64,
        /// Maximum amount of memory that a smart contract can use
        pub memory: NonZeroU64,
    }

    /// Single smart contract parameter
    ///
    /// Check [`SmartContractParameters`] for more details
    #[derive(
        Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Serialize, IntoSchema,
    )]
    pub enum SmartContractParameter {
        Fuel(NonZeroU64),
        Memory(NonZeroU64),
    }

    /// Blockchain specific parameter defined in the executor
    #[derive(
        Debug, Display, Clone, IdEqOrdHash, Decode, Encode, Deserialize, Serialize, IntoSchema,
    )]
    #[ffi_type]
    #[display(fmt = "{id}({payload})")]
    pub struct CustomParameter {
        /// Unique id of the parameter.
        pub id: CustomParameterId,
        /// Payload containing actual value.
        ///
        /// It is JSON-encoded, and its structure must correspond to the structure of
        /// the type defined in [`crate::executor::ExecutorDataModel`].
        pub payload: JsonString,
    }

    /// Set of all current blockchain parameter values
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Default,
        Getters,
        CopyGetters,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    pub struct Parameters {
        /// Sumeragi parameters
        #[getset(get_copy = "pub")]
        pub sumeragi: SumeragiParameters,
        /// Block parameters
        #[getset(get_copy = "pub")]
        pub block: BlockParameters,
        /// Transaction parameters
        #[getset(get_copy = "pub")]
        pub transaction: TransactionParameters,
        /// Executor parameters
        #[getset(get_copy = "pub")]
        pub executor: SmartContractParameters,
        /// Smart contract parameters
        #[getset(get_copy = "pub")]
        pub smart_contract: SmartContractParameters,
        /// Collection of blockchain specific parameters
        #[getset(get = "pub")]
        pub custom: CustomParameters,
    }

    /// Single blockchain parameter.
    ///
    /// Check [`Parameters`] for more details
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        EnumDiscriminants,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[ffi_type(opaque)]
    pub enum Parameter {
        Sumeragi(SumeragiParameter),
        Block(BlockParameter),
        Transaction(TransactionParameter),
        SmartContract(SmartContractParameter),
        Executor(SmartContractParameter),
        Custom(CustomParameter),
    }
}

impl core::fmt::Display for Parameter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sumeragi(v) => core::fmt::Display::fmt(&v, f),
            Self::Block(v) => core::fmt::Display::fmt(&v, f),
            Self::Transaction(v) => core::fmt::Display::fmt(&v, f),
            Self::SmartContract(v) | Self::Executor(v) => core::fmt::Display::fmt(&v, f),
            Self::Custom(v) => write!(f, "{}({})", v.id, v.payload),
        }
    }
}

impl SumeragiParameters {
    /// Maximal amount of time (in milliseconds) a peer will wait before forcing creation of a new block.
    ///
    /// A block is created if this limit or [`BlockParameters::max_transactions`] limit is reached,
    /// whichever comes first. Regardless of the limits, an empty block is never created.
    pub fn block_time(&self) -> Duration {
        Duration::from_millis(self.block_time_ms)
    }

    /// Time (in milliseconds) a peer will wait for a block to be committed.
    ///
    /// If this period expires the block will request a view change
    pub fn commit_time(&self) -> Duration {
        Duration::from_millis(self.commit_time_ms)
    }

    /// Maximal amount of time it takes to commit a block
    #[cfg(feature = "transparent_api")]
    pub fn pipeline_time(&self) -> Duration {
        self.block_time() + self.commit_time()
    }

    /// Estimation of consensus duration
    #[cfg(feature = "transparent_api")]
    pub fn consensus_estimation(&self) -> Duration {
        self.block_time() + (self.commit_time() / 2)
    }
}

impl Default for SumeragiParameters {
    fn default() -> Self {
        pub const DEFAULT_BLOCK_TIME: u64 = 2_000;
        pub const DEFAULT_COMMIT_TIME: u64 = 4_000;

        Self {
            block_time_ms: DEFAULT_BLOCK_TIME,
            commit_time_ms: DEFAULT_COMMIT_TIME,
        }
    }
}
impl Default for BlockParameters {
    fn default() -> Self {
        /// Default value for [`Parameters::MaxTransactionsInBlock`]
        pub const DEFAULT_TRANSACTIONS_IN_BLOCK: NonZeroU64 = nonzero!(2_u64.pow(9));

        Self::new(DEFAULT_TRANSACTIONS_IN_BLOCK)
    }
}

impl Default for TransactionParameters {
    fn default() -> Self {
        const DEFAULT_INSTRUCTION_NUMBER: NonZeroU64 = nonzero!(2_u64.pow(12));
        const DEFAULT_SMART_CONTRACT_SIZE: NonZeroU64 = nonzero!(4 * 2_u64.pow(20));

        Self::new(DEFAULT_INSTRUCTION_NUMBER, DEFAULT_SMART_CONTRACT_SIZE)
    }
}

impl Default for SmartContractParameters {
    fn default() -> Self {
        const DEFAULT_FUEL: NonZeroU64 = nonzero!(55_000_000_u64);
        const DEFAULT_MEMORY: NonZeroU64 = nonzero!(55_000_000_u64);

        Self {
            fuel: DEFAULT_FUEL,
            memory: DEFAULT_MEMORY,
        }
    }
}

impl Parameters {
    /// Convert [`Self`] into iterator of individual parameters
    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + '_ {
        self.sumeragi
            .parameters()
            .map(Parameter::Sumeragi)
            .chain(self.block.parameters().map(Parameter::Block))
            .chain(self.transaction.parameters().map(Parameter::Transaction))
            .chain(self.executor.parameters().map(Parameter::Executor))
            .chain(
                self.smart_contract
                    .parameters()
                    .map(Parameter::SmartContract),
            )
            .chain(self.custom.values().cloned().map(Parameter::Custom))
    }
}

impl SumeragiParameters {
    /// Construct [`Self`]
    pub fn new(block_time: Duration, commit_time: Duration) -> Self {
        Self {
            block_time_ms: block_time
                .as_millis()
                .try_into()
                .expect("INTERNAL BUG: Time should fit into u64"),
            commit_time_ms: commit_time
                .as_millis()
                .try_into()
                .expect("INTERNAL BUG: Time should fit into u64"),
        }
    }

    /// Convert [`Self`] into iterator of individual parameters
    pub fn parameters(&self) -> impl Iterator<Item = SumeragiParameter> {
        [
            SumeragiParameter::BlockTimeMs(self.block_time_ms),
            SumeragiParameter::CommitTimeMs(self.commit_time_ms),
        ]
        .into_iter()
    }
}

impl BlockParameters {
    /// Construct [`Self`]
    pub const fn new(max_transactions: NonZeroU64) -> Self {
        Self { max_transactions }
    }

    /// Convert [`Self`] into iterator of individual parameters
    pub fn parameters(&self) -> impl Iterator<Item = BlockParameter> {
        [BlockParameter::MaxTransactions(self.max_transactions)].into_iter()
    }
}

impl TransactionParameters {
    /// Construct [`Self`]
    pub const fn new(max_instructions: NonZeroU64, smart_contract_size: NonZeroU64) -> Self {
        Self {
            max_instructions,
            smart_contract_size,
        }
    }

    /// Convert [`Self`] into iterator of individual parameters
    pub fn parameters(&self) -> impl Iterator<Item = TransactionParameter> {
        [
            TransactionParameter::MaxInstructions(self.max_instructions),
            TransactionParameter::SmartContractSize(self.smart_contract_size),
        ]
        .into_iter()
    }
}

impl SmartContractParameters {
    /// Convert [`Self`] into iterator of individual parameters
    pub fn parameters(&self) -> impl Iterator<Item = SmartContractParameter> {
        [
            SmartContractParameter::Fuel(self.fuel),
            SmartContractParameter::Memory(self.memory),
        ]
        .into_iter()
    }
}

impl CustomParameterId {
    /// Getter for name
    pub fn name(&self) -> &Name {
        &self.0
    }
}

impl CustomParameter {
    /// Constructor
    pub fn new(id: CustomParameterId, payload: impl Into<JsonString>) -> Self {
        Self {
            id,
            payload: payload.into(),
        }
    }

    /// Getter
    // TODO: derive with getset once FFI impl is fixed
    pub fn payload(&self) -> &JsonString {
        &self.payload
    }
}

mod candidate {
    use core::num::NonZeroUsize;

    use parity_scale_codec::{Decode, Input};
    use serde::Deserialize;

    use super::*;

    #[derive(Decode, Deserialize)]
    enum TransactionParameterCandidate {
        MaxInstructions(NonZeroU64),
        SmartContractSize(NonZeroU64),
    }

    #[derive(Decode, Deserialize)]
    struct TransactionParametersCandidate {
        max_instructions: NonZeroU64,
        smart_contract_size: NonZeroU64,
    }

    #[derive(Decode, Deserialize)]
    enum BlockParameterCandidate {
        MaxTransactions(NonZeroU64),
    }

    #[derive(Decode, Deserialize)]
    struct BlockParametersCandidate {
        max_transactions: NonZeroU64,
    }

    #[derive(Decode, Deserialize)]
    enum SmartContractParameterCandidate {
        Fuel(NonZeroU64),
        Memory(NonZeroU64),
    }

    #[derive(Decode, Deserialize)]
    struct SmartContractParametersCandidate {
        fuel: NonZeroU64,
        memory: NonZeroU64,
    }

    impl BlockParameterCandidate {
        fn validate(self) -> Result<BlockParameter, &'static str> {
            Ok(match self {
                Self::MaxTransactions(max_transactions) => {
                    let _ = NonZeroUsize::try_from(max_transactions)
                        .map_err(|_| "BlockParameter::MaxTransactions exceeds usize::MAX")?;

                    BlockParameter::MaxTransactions(max_transactions)
                }
            })
        }
    }

    impl BlockParametersCandidate {
        fn validate(self) -> Result<BlockParameters, &'static str> {
            let _ = NonZeroUsize::try_from(self.max_transactions)
                .map_err(|_| "BlockParameters::max_transactions exceeds usize::MAX")?;

            Ok(BlockParameters {
                max_transactions: self.max_transactions,
            })
        }
    }

    impl TransactionParameterCandidate {
        fn validate(self) -> Result<TransactionParameter, &'static str> {
            Ok(match self {
                Self::MaxInstructions(max_instructions) => {
                    let _ = NonZeroUsize::try_from(max_instructions)
                        .map_err(|_| "TransactionParameter::MaxInstructions exceeds usize::MAX")?;
                    TransactionParameter::MaxInstructions(max_instructions)
                }
                Self::SmartContractSize(smart_contract_size) => {
                    let _ = NonZeroUsize::try_from(smart_contract_size).map_err(|_| {
                        "TransactionParameter::SmartContractSize exceeds usize::MAX"
                    })?;
                    TransactionParameter::SmartContractSize(smart_contract_size)
                }
            })
        }
    }

    impl TransactionParametersCandidate {
        fn validate(self) -> Result<TransactionParameters, &'static str> {
            let _ = NonZeroUsize::try_from(self.max_instructions)
                .map_err(|_| "TransactionParameters::max_instructions exceeds usize::MAX")?;

            let _ = NonZeroUsize::try_from(self.smart_contract_size)
                .map_err(|_| "TransactionParameters::smart_contract_size exceeds usize::MAX")?;

            Ok(TransactionParameters {
                max_instructions: self.max_instructions,
                smart_contract_size: self.smart_contract_size,
            })
        }
    }

    impl SmartContractParameterCandidate {
        fn validate(self) -> Result<SmartContractParameter, &'static str> {
            Ok(match self {
                Self::Fuel(fuel) => SmartContractParameter::Fuel(fuel),
                Self::Memory(memory) => {
                    NonZeroUsize::try_from(memory)
                        .map_err(|_| "SmartContractParameter::Memory exceeds usize::MAX")?;
                    SmartContractParameter::Memory(memory)
                }
            })
        }
    }

    impl SmartContractParametersCandidate {
        fn validate(self) -> Result<SmartContractParameters, &'static str> {
            let _ = NonZeroUsize::try_from(self.memory)
                .map_err(|_| "SmartContractParameters::memory exceeds usize::MAX")?;

            Ok(SmartContractParameters {
                fuel: self.fuel,
                memory: self.memory,
            })
        }
    }

    impl Decode for BlockParameter {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            BlockParameterCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }

    impl<'de> Deserialize<'de> for BlockParameter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            BlockParameterCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }

    impl Decode for BlockParameters {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            BlockParametersCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }

    impl<'de> Deserialize<'de> for BlockParameters {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            BlockParametersCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }

    impl Decode for TransactionParameter {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            TransactionParameterCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }

    impl<'de> Deserialize<'de> for TransactionParameter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            TransactionParameterCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }

    impl Decode for TransactionParameters {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            TransactionParametersCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }

    impl<'de> Deserialize<'de> for TransactionParameters {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            TransactionParametersCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }

    impl Decode for SmartContractParameter {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            SmartContractParameterCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }
    impl<'de> Deserialize<'de> for SmartContractParameter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            SmartContractParameterCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }

    impl Decode for SmartContractParameters {
        fn decode<I: Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
            SmartContractParametersCandidate::decode(input)?
                .validate()
                .map_err(Into::into)
        }
    }
    impl<'de> Deserialize<'de> for SmartContractParameters {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error as _;

            SmartContractParametersCandidate::deserialize(deserializer)?
                .validate()
                .map_err(D::Error::custom)
        }
    }
}
pub mod prelude {
    //! Prelude: re-export of most commonly used traits, structs and macros in this crate.

    pub use super::{Parameter, Parameters, SmartContractParameters, TransactionParameters};
}
