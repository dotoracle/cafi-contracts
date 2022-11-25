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
    InvalidWrappedToken = 101,
    OnlyOwnerCanRevoke = 102,
    UnsupportedToken = 103,
    AskForMore = 104,    
    OnlyBidderCanRevoke = 105,
    MissingFeePortion = 106,
    InvalidFeePortion = 107,
    NftIsNotApproved = 108,
    BidTooLow = 109,
    BidInactive = 110,
    OnlyOfferorCanRevoke = 111,
    OfferInactive = 112,
    OnlyBidderCanIncreaseBid = 113,
    FeeTooHigh = 114,
    OnlyOwnerCanOffer = 115,
    MissingContractOwner = 116,
    ContractIllFormatted = 117,
    MissingWcsprContract = 118,
    InvalidWcsprContract = 119,
    MissingFeeReceiver = 120,
    InvalidFeeReceiver = 121,
    MissingOfferer = 122,
    AlreadyMakeOffer = 123,
    MissingRoyaltyFee = 124,
    InvalidRoyaltyFee = 125,
    MissingIsRoyalty = 126,
    InvalidIsRoyalty = 127,
    SameIsRoyalty = 128,
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}
