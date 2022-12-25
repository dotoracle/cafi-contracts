//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
/// Such a user error should be in the range `[0..(u16::MAX - 4)]` (i.e. [0, 65532]) to avoid
/// conflicting with the other `Error` variants.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    InvalidContext = 0,
    /// Spender does not have enough balance.
    InsufficientBalance = 1,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance = 2,
    /// Operation would cause an integer overflow.
    Overflow = 3,
    /// User error.
    FailedToGetArgBytes = 200,
    FailedToCreateDictionary = 201,
    MissingContractOwner = 202,
    InvalidContractOwner = 203,
    MissingSupportedToken = 204,
    InvalidSupportedToken = 205,
    CannotGetWhitelistAddrressArg = 206,
    MissingEnabled = 207,
    InvalidEnabled = 208,
    CannotGetEnabled = 209,
    SameEnabledValue = 210,
    MissingDecimals = 211,
    InvalidDecimals = 212,
    SameDecimalsValue = 213,
    MissingFee = 214,
    InvalidFee = 215,
    MissingFeeReceiver = 216,
    InvalidFeeReceiver = 217,
    InvalidCaller = 218,
    ContractAlreadyInitialized = 219,
    MissingContractName = 220,
    InvalidContractName = 221,


}
const ERROR_INVALID_CONTEXT: u16 = u16::MAX;
const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
const ERROR_OVERFLOW: u16 = u16::MAX - 3;

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}
