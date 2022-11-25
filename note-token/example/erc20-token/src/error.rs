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
pub enum ErrorERC20 {
    /// ERC20 contract called from within an invalid context.
    InvalidContext = 0,
    /// Spender does not have enough balance.
    InsufficientBalance =1,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance =2,
    /// Operation would cause an integer overflow.
    Overflow =3,
    /// User error.
    FailedToGetArgBytes = 100,
    MissingOwner = 101,
    InvalidOwner = 102,
    MissingMintAmount = 103,
    InvalidMintAmount = 104,
    FailCallToMint = 105,
    FailCallToBurn = 106,

}

const ERROR_INVALID_CONTEXT: u16 = u16::MAX;
const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
const ERROR_OVERFLOW: u16 = u16::MAX - 3; 

// impl From<Error> for ApiError {
//     fn from(error: Error) -> Self {
//         let user_error = match error {
//             Error::InvalidContext => ERROR_INVALID_CONTEXT,
//             Error::InsufficientBalance => ERROR_INSUFFICIENT_BALANCE,
//             Error::InsufficientAllowance => ERROR_INSUFFICIENT_ALLOWANCE,
//             Error::Overflow => ERROR_OVERFLOW,
//             ApiError::User(user_error as u16) => user_error,
//         };
//         // ApiError::User(e as u16)
//         ApiError::User(user_error)
//     }
// }
impl From<ErrorERC20> for ApiError {
    fn from(e: ErrorERC20) -> Self {
        ApiError::User(e as u16)
    }
}