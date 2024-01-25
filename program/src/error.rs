//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the TokenAmm program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AmmError {
    // 0
    /// The account cannot be initialized because it is already being used.
    /// The pc vault provided doesn't match the pc vault in the AmmInfo.
    #[error("InvalidPCVault")]
    InvalidPCVault,
    /// The token_lp provided doesn't match the token_lp in the AmmInfo.
    #[error("InvalidTokenLP")]
    InvalidTokenLP,
    /// The dest_token_coin provided doesn't match the dest_token_coin in WithdrawTokenInfo.
    #[error("InvalidDestTokenCoin")]
    InvalidDestTokenCoin,
    /// The dest_token_pc provided doesn't match the dest_token_pc in WithdrawTokenInfo.
    #[error("InvalidDestTokenPC")]
    InvalidDestTokenPC,
    /// The pool_mint provided doesn't match the pool_mint in the AmmInfo.
    #[error("InvalidPoolMint")]
    InvalidPoolMint,

    // 10
    /// The open_orders provided doesn't match the open_orders in in the AmmInfo.
    #[error("InvalidOpenOrders")]
    InvalidOpenOrders,
    /// The market provided doesn't match the market in the AmmInfo.
    #[error("InvalidMarket")]
    InvalidMarket,
    /// The market program provided doesn't match the market program in the AmmInfo.
    #[error("InvalidMarketProgram")]
    InvalidMarketProgram,
    /// The target_orders provided doesn't match the target_orders in the AmmInfo.
    #[error("InvalidTargetOrders")]
    InvalidTargetOrders,
    /// The Account provided must be writeable.
    #[error("AccountNeedWriteable")]
    AccountNeedWriteable,

    // 15
    /// The Account provided must be readonly.
    #[error("AccountNeedReadOnly")]
    AccountNeedReadOnly,
    /// The token_coin's mint provided doesn't match the coin_mint's key.
    #[error("InvalidCoinMint")]
    InvalidCoinMint,
    /// The token_pc's mint provided doesn't match the pc_mint's key.
    #[error("InvalidPCMint")]
    InvalidPCMint,
    /// The owner of the input isn't set to the program address generated by the program.
    #[error("InvalidOwner")]
    InvalidOwner,
    /// The initialized pool had a non zero supply.
    #[error("InvalidSupply")]
    InvalidSupply,

    // 20
    /// The initialized token has a delegate.
    #[error("InvalidDelegate")]
    InvalidDelegate,
    /// Invalid Sign Account
    #[error("Invalid Sign Account")]
    InvalidSignAccount,
    /// The amm status is invalid.
    #[error("InvalidStatus")]
    InvalidStatus,
    /// Invalid instruction number passed in
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// The number of account provided does not match the expectations
    #[error("Wrong accounts number")]
    WrongAccountsNumber,

    // 25
    /// The target account owner is not match with this program
    #[error("The target account owner is not match with this program")]
    InvalidTargetAccountOwner,
    /// The owner saved in target is not match with this amm pool
    #[error("The owner saved in target is not match with this amm pool")]
    InvalidTargetOwner,
    /// The amm account owner is not match with this program"
    #[error("The amm account owner is not match with this program")]
    InvalidAmmAccountOwner,
    /// The params set is invalid
    #[error("Params Set is invalid")]
    InvalidParamsSet,
    /// The params input is invalid.
    #[error("InvalidInput")]
    InvalidInput,

    // 30
    /// instruction exceeds desired slippage limit
    #[error("instruction exceeds desired slippage limit")]
    ExceededSlippage,
    /// The calculation exchange rate failed.
    #[error("CalculationExRateFailure")]
    CalculationExRateFailure,
    /// Checked_Sub Overflow
    #[error("Checked_Sub Overflow")]
    CheckedSubOverflow,
    /// Checked_Add Overflow
    #[error("Checked_Add Overflow")]
    CheckedAddOverflow,
    /// Checked_Mul Overflow
    #[error("Checked_Mul Overflow")]
    CheckedMulOverflow,

    // 35
    /// Checked_Div Overflow
    #[error("Checked_Div Overflow")]
    CheckedDivOverflow,
    /// Empty Funds
    #[error("Empty Funds")]
    CheckedEmptyFunds,
    /// Calc pnl error
    #[error("Calc pnl error")]
    CalcPnlError,
    /// InvalidSplTokenProgram
    #[error("InvalidSplTokenProgram")]
    InvalidSplTokenProgram,
    /// TakePnlError
    #[error("Take Pnl error")]
    TakePnlError,

    // 40
    /// Insufficient funds
    #[error("Insufficient funds")]
    InsufficientFunds,
    /// ConversionFailure
    #[error("Conversion to u64 failed with an overflow or underflow")]
    ConversionFailure,
    /// The user token input does not match amm
    #[error("user token input does not match amm")]
    InvalidUserToken,
    // The srm_token's mint provided doesn't match the pc_mint's key.
    #[error("InvalidSrmMint")]
    InvalidSrmMint,
    /// The srm_token provided doesn't match the srm_token in the program.
    #[error("InvalidSrmToken")]
    InvalidSrmToken,

    // 45
    /// TooManyOpenOrders
    #[error("TooManyOpenOrders")]
    TooManyOpenOrders,
    /// OrderAtSlotIsPlaced
    #[error("OrderAtSlotIsPlaced")]
    OrderAtSlotIsPlaced,
    /// InvalidSysProgramAddress
    #[error("InvalidSysProgramAddress")]
    InvalidSysProgramAddress,
    /// The provided fee does not match the program owner's constraints
    #[error("The provided fee does not match the program owner's constraints")]
    InvalidFee,
    /// Repeat create amm about market
    #[error("Repeat create amm about market")]
    RepeatCreateAmm,

    // 50
    /// Not allow Zero LP
    #[error("Not allow Zero LP")]
    NotAllowZeroLP,
    /// The provided token account has a close authority.
    #[error("Token account has a close authority")]
    InvalidCloseAuthority,
    /// The pool token mint has a freeze authority.
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority,
    // The referrer_pc_wallet's mint provided doesn't match the pc_mint's key.
    #[error("InvalidReferPCMint")]
    InvalidReferPCMint,
    /// InvalidConfigAccount
    #[error("InvalidConfigAccount")]
    InvalidConfigAccount,

    // 55
    /// RepeatCreateConfigAccount
    #[error("Repeat create config account")]
    RepeatCreateConfigAccount,
    /// MarketLotSizeIsTooLarge
    #[error("Market lotSize is too large")]
    MarketLotSizeIsTooLarge,
    /// Init lp amount is too less.
    #[error("Init lp amount is too less(Because 10**lp_decimals amount lp will be locked)")]
    InitLpAmountTooLess,
    /// Unknown Amm Error
    #[error("Unknown Amm Error")]
    UnknownAmmError,
}

impl From<AmmError> for ProgramError {
    fn from(e: AmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for AmmError {
    fn type_of() -> &'static str {
        "Amm Error"
    }
}

impl PrintProgramError for AmmError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            AmmError::AlreadyInUse => msg!("Error: AlreadyInUse"),
            AmmError::InvalidProgramAddress => msg!("Error: InvalidProgramAddress"),
            AmmError::ExpectedMint => msg!("Error: ExpectedMint"),
            AmmError::ExpectedAccount => msg!("Error: ExpectedAccount"),
            AmmError::InvalidCoinVault => msg!("Error: InvalidCoinVault"),

            AmmError::InvalidPCVault => msg!("Error: InvalidPCVault"),
            AmmError::InvalidTokenLP => msg!("Error: InvalidTokenLP"),
            AmmError::InvalidDestTokenCoin => msg!("Error: InvalidDestTokenCoin"),
            AmmError::InvalidDestTokenPC => msg!("Error: InvalidDestTokenPC"),
            AmmError::InvalidPoolMint => msg!("Error: InvalidPoolMint"),
            AmmError::InvalidOpenOrders => msg!("Error: InvalidOpenOrders"),
            AmmError::InvalidMarket => msg!("Error: InvalidMarket"),
            AmmError::InvalidMarketProgram => msg!("Error: InvalidMarketProgram"),

            AmmError::InvalidTargetOrders => msg!("Error: InvalidTargetOrders"),
            AmmError::AccountNeedWriteable => msg!("Error: AccountNeedWriteable"),
            AmmError::AccountNeedReadOnly => msg!("Error: AccountNeedReadOnly"),
            AmmError::InvalidCoinMint => msg!("Error: InvalidCoinMint"),
            AmmError::InvalidPCMint => msg!("Error: InvalidPCMint"),

            AmmError::InvalidOwner => msg!("Error: InvalidOwner"),
            AmmError::InvalidSupply => msg!("Error: InvalidSupply"),
            AmmError::InvalidDelegate => msg!("Error: InvalidDelegate"),
            AmmError::InvalidSignAccount => msg!("Error: Invalid Sign Account"),
            AmmError::InvalidStatus => msg!("Error: InvalidStatus"),

            AmmError::InvalidInstruction => msg!("Error: InvalidInstruction"),
            AmmError::WrongAccountsNumber => msg!("Error: WrongAccountsNumber"),
            AmmError::InvalidTargetAccountOwner => msg!("Error: The target account owner is not match with this program"),
            AmmError::InvalidTargetOwner => msg!("Error: The owner saved in target is not match with this amm pool"),
            AmmError::InvalidAmmAccountOwner => msg!("Error: The amm account owner is not match with this program"),

            AmmError::InvalidParamsSet => msg!("Error: Params Set is Invalid"),
            AmmError::InvalidInput => msg!("Error: InvalidInput"),
            AmmError::ExceededSlippage => msg!("Error: exceeds desired slippage limit"),
            AmmError::CalculationExRateFailure => msg!("Error: CalculationExRateFailure"),
            AmmError::CheckedSubOverflow => msg!("Error: Checked_Sub Overflow"),

            AmmError::CheckedAddOverflow => msg!("Error: Checked_Add Overflow"),
            AmmError::CheckedMulOverflow => msg!("Error: Checked_Mul Overflow"),
            AmmError::CheckedDivOverflow => msg!("Error: Checked_Div Overflow"),
            AmmError::CheckedEmptyFunds => msg!("Error: CheckedEmptyFunds"),
            AmmError::CalcPnlError => msg!("Error: CalcPnlError"),

            AmmError::InvalidSplTokenProgram => msg!("Error: InvalidSplTokenProgram"),
            AmmError::TakePnlError => msg!("Error: TakePnlError"),
            AmmError::InsufficientFunds => msg!("Error: insufficient funds"),
            AmmError::ConversionFailure => msg!("Error: Conversion to or from u64 failed."),
            AmmError::InvalidUserToken => msg!("Error: User token input does not match amm"),

            AmmError::InvalidSrmMint => msg!("Error: InvalidSrmMint"),
            AmmError::InvalidSrmToken => msg!("Error: InvalidSrmToken"),
            AmmError::TooManyOpenOrders => msg!("Error: TooManyOpenOrders"),
            AmmError::OrderAtSlotIsPlaced => msg!("Error: OrderAtSlotIsPlaced"),
            AmmError::InvalidSysProgramAddress => msg!("Error: InvalidSysProgramAddress"),

            AmmError::InvalidFee => msg!("Error: InvalidFee"),
            AmmError::RepeatCreateAmm => msg!("Error: RepeatCreateAmm"),
            AmmError::NotAllowZeroLP => msg!("Error: NotAllowZeroLP"),
            AmmError::InvalidCloseAuthority => msg!("Error: Token account has a close authority"),
            AmmError::InvalidFreezeAuthority => {
                msg!("Error: Pool token mint has a freeze authority")
            }
            AmmError::InvalidReferPCMint => msg!("Error: InvalidReferPCMint"),
            AmmError::InvalidConfigAccount => msg!("Error: InvalidConfigAccount"),
            AmmError::RepeatCreateConfigAccount => msg!("Error: RepeatCreateConfigAccount"),
            AmmError::MarketLotSizeIsTooLarge => msg!("Error: Market lotSize is too large"),
            AmmError::InitLpAmountTooLess => msg!("Error: Init lp amount is too less(Because 10**lp_decimals amount lp will be locked)"),
            AmmError::UnknownAmmError => msg!("Error: UnknownAmmError"),
        }
    }
}
