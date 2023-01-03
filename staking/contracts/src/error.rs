use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    InvalidAccount = 1,
    MissingInstaller = 2,
    InvalidContext = 3,
    InvalidIdentifierMode = 4,
    MissingTokenID = 5,
    InvalidTokenIdentifier = 6,
    FailedToGetArgBytes = 7,
    InvalidContractOwner = 8,
    RequestIdIlledFormat = 9,
    FailedToCreateDictionary = 10,
    RequestIdRepeated = 11,
    MissingKey = 12,
    SerilizationError = 13,
    UnlockIdRepeated = 14,
    FailedToCreateDictionaryUnlockIds = 15,
    ContractAlreadyInitialized = 16,
    CallerMustBeAccountHash = 17,
    TooManyTokenIds = 18,
    UnlockIdIllFormatted = 19,
    TxHashUnlockIdIllFormatted = 20,
    InvalidDev = 100,
    MisingPoolInfo = 101,
    InvalidPoolInfo = 102,
    MissingContractOwner = 103,
    PoolAlreadyInit = 104,
    MissingUser = 105,
    InvalidUser = 106,
    MissingLpContractHash = 107,
    InvalidLpContractHash = 108,
    MissingAmount = 109,
    InvalidAmount = 110,
    MissingStakeDuration = 111,
    InvalidStakeDuration = 112,
    MissingPoolList = 113,
    InvalidPoolList = 114,
    MissingPoolId = 115,
    InvalidPoolId = 116,
    InvalidTimestamps = 117,
    MissingRewardToken = 118,
    InvalidRewardToken = 119,
    MissingTotalAllocPoint = 120,
    InvalidTotalAllocPoint = 121,
    MissingRewardPerSecond = 122,
    InvalidRewardPerSecond = 123,

    
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}
