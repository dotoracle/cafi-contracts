use alloc::{
    vec::*,
    vec
};
use crate::alloc::string::ToString;
use casper_contract::{
    contract_api::{
        runtime,
        runtime::print
    }
};
use casper_types::{
    runtime_args, ContractHash,
    RuntimeArgs, U256, Key
};
use crate::events::{self, CAFIDexEvent};

use crate::ampl::{self, get_a_precise };

use crate::helpers::{self, get_self_key, require};
use crate::error::Error;

use crate::math_utils::{ self, mul_div };
use crate::structs::*;
use crate::erc20_helpers;

pub const POOL_PRECISION_DECIMALS: u8 = 18;
pub const FEE_DENOMINATOR: u128 = 10000000000; // 10^10
pub const MAX_SWAP_FEE: u128 = 100000000; // 10^8
pub const MAX_ADMIN_FEE: u128 = 10000000000; // 10^10
pub const MAX_LOOP_LIMIT: u64 = 256; 

fn get_a_precise_internal(swap: &Swap) -> u128 {
    get_a_precise(swap)
}

/**
* @notice Calculate the dy, the amount of selected token that user receives and
* the fee of withdrawing in one token
* @param tokenAmount the amount to withdraw in the pool's precision
* @param tokenIndex which token will be withdrawn
* @param self Swap struct to read from
* @return the amount of token user will receive
*/
pub fn calculate_withdraw_one_token(
    swap: &Swap,
    token_amount: u128,
    token_index: usize
) -> u128 {
    let lp_supply : U256 = runtime::call_contract(
        ContractHash::new(swap.lp_token.into_hash().unwrap()),
        "total_supply",
        runtime_args! {},
    );
    let (available_token_amount, _) = _calculate_withdraw_one_token(
        swap,
        token_amount,
        token_index,
        lp_supply.as_u128()
    );
    return available_token_amount;
}

fn _calculate_withdraw_one_token(
    swap: &Swap,
    token_amount: u128,
    token_index: usize,
    total_supply: u128
) -> (u128, u128) {
    let (dy, new_y, current_y) = calculate_withdraw_one_token_dy(
        swap,
        token_index,
        token_amount,
        total_supply
    );

    // dy_0 (without fees)
    // dy, dy_0 - dy

    let dy_swap_fee = (current_y - new_y) / (swap.token_precision_multipliers[token_index]) - dy;

    return (dy, dy_swap_fee);
}

/**
* @notice Calculate the dy of withdrawing in one token
* @param self Swap struct to read from
* @param tokenIndex which token will be withdrawn
* @param tokenAmount the amount to withdraw in the pools precision
* @return the d and the new y after withdrawing one token
*/
fn calculate_withdraw_one_token_dy(
    swap: &Swap,
    token_index: usize,
    token_amount: u128,
    total_supply: u128
) -> (
    u128,
    u128,
    u128
    )
{
    // Get the current D, then solve the stableswap invariant
    // y_i for D - tokenAmount
    let xp = _xp2(swap);

    require(token_index < xp.len(), Error::TokenIndexOutOfRange);

    let mut v = CalculateWithdrawOneTokenDYInfo{ 
        d0: 0,
        d1: 0,
        new_y: 0,
        fee_per_token: 0,
        precise_a: 0
    };
    v.precise_a = get_a_precise_internal(swap);
    v.d0 = get_d(&xp, v.precise_a);
    v.d1 = v.d0 - mul_div(token_amount, v.d0, total_supply);

    require(token_amount <= xp[token_index], Error::WithdrawExceedAvailable);

    v.new_y = get_yd(v.precise_a, token_index, &xp, v.d1);

    let mut xp_reduced = vec![0; xp.len()];

    v.fee_per_token = _fee_per_token(swap.swap_fee.into(), xp.len() as u128);
    for i in 0..xp.len() {
        let xpi = xp[i];
        // if i == tokenIndex, dxExpected = xp[i] * d1 / d0 - newY
        // else dxExpected = xp[i] - (xp[i] * d1 / d0)
        // xpReduced[i] -= dxExpected * fee / FEE_DENOMINATOR
        xp_reduced[i] = xpi - (
            mul_div((
                if i == token_index {
                    mul_div(xpi, v.d1, v.d0) - v.new_y
                } else {
                    xpi - mul_div(xpi, v.d1, v.d0)
                }
            ), v.fee_per_token, FEE_DENOMINATOR)
        );
    }

    let mut dy = xp_reduced[token_index] - (
        get_yd(v.precise_a, token_index, &xp_reduced, v.d1)
    );
    dy = (dy - 1) / (swap.token_precision_multipliers[token_index]);

    return (dy, v.new_y, xp[token_index]);
}

/**
* @notice Calculate the price of a token in the pool with given
* precision-adjusted balances and a particular D.
*
* @dev This is accomplished via solving the invariant iteratively.
* See the StableSwap paper and Curve.fi implementation for further details.
*
* x_1**2 + x1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
* x_1**2 + b*x_1 = c
* x_1 = (x_1**2 + c) / (2*x_1 + b)
*
* @param a the amplification coefficient * n * (n - 1). See the StableSwap paper for details.
* @param tokenIndex Index of token we are calculating for.
* @param xp a precision-adjusted set of pool balances. Array should be
* the same cardinality as the pool.
* @param d the stableswap invariant
* @return the price of the token, in the same precision as in xp
*/
fn get_yd(
    a: u128,
    token_index: usize,
    xp: &Vec<u128>,
    d: u128
) -> u128 {
    let num_tokens = xp.len();
    require(token_index < num_tokens, Error::TokenNotFound);

    let mut c = d;
    let mut s = 0u128;
    let n_a = a * (num_tokens as u128);

    for i in 0..num_tokens {
        if i != token_index {
            s = s + xp[i];
            c = mul_div(c, d, (xp[i] * (num_tokens as u128)));
            // If we were to protect the division loss we would have to keep the denominator separate
            // and divide at the end. However this leads to overflow with large numTokens or/and D.
            // c = c * D * D * D * ... overflow!
        }
    }
    c = mul_div(c, d * ampl::A_PRECISION, (n_a * (num_tokens as u128)));

    let b = s + mul_div(d, ampl::A_PRECISION, n_a);
    let mut y_prev
    ;
    let mut y = d;
    for _i in 0..MAX_LOOP_LIMIT {
        y_prev = y;
        y = ((U256::from(y) * U256::from(y) + U256::from(c)) / U256::from(y * 2 + b - d)).as_u128();
        if math_utils::within1(y, y_prev) {
            return y;
        }
    }
    runtime::revert(Error::ApproximationNotConverge);
}


 /**
* @notice Get D, the StableSwap invariant, based on a set of balances and a particular A.
* @param xp a precision-adjusted set of pool balances. Array should be the same cardinality
* as the pool.
* @param a the amplification coefficient * n * (n - 1) in A_PRECISION.
* See the StableSwap paper for details
* @return the invariant, at the precision of the pool
*/
fn get_d(xp: &Vec<u128>, a: u128) -> u128
{
    let num_tokens = xp.len();
    let mut s = 0u128;
    for i in 0..num_tokens {
        s = s + xp[i];
    }
    if s == 0 {
        return 0;
    }

    let mut prev_d;
    let mut d = s;
    let n_a = a * (num_tokens as u128);
    for _i in 0..MAX_LOOP_LIMIT {
        let mut d_p = d;
        for j in 0..num_tokens {
            d_p = mul_div(d_p, d, (xp[j] * (num_tokens as u128)));
            // If we were to protect the division loss we would have to keep the denominator separate
            // and divide at the end. However this leads to overflow with large numTokens or/and D.
            // dP = dP * D * D * D * ... overflow!
        }

        prev_d = d;
        d = mul_div((mul_div(n_a, s, ampl::A_PRECISION) + d_p * (num_tokens as u128)), d, (
                mul_div((n_a - ampl::A_PRECISION), d, ampl::A_PRECISION) + ((num_tokens + 1) as u128) * d_p
            ));
        // print(&d.to_string());
        // print(&prev_d.to_string());
        if math_utils::within1(d, prev_d) {
            return d;
        }
    }

    // Convergence should occur in 4 loops or less. If this is reached, there may be something wrong
    // with the pool. If this were to occur repeatedly, LPs should withdraw via `removeLiquidity()`
    // function which does not rely on D.
    runtime::revert(Error::DNotConverge);
}

fn _xp(
    balances: &Vec<u128>,
    precision_multipliers: &Vec<u128>
) -> Vec<u128> {
    let num_tokens = balances.len();
    require(
        num_tokens == precision_multipliers.len(),
        Error::BalancesMustMatchMultipliers
    );
    let mut xp = Vec::new();
    for i in 0..num_tokens {
        xp.push(balances[i] * precision_multipliers[i]);
    }
    return xp;
}

fn _xp2(swap: &Swap) -> Vec<u128> {
    _xp(&swap.balances, &swap.token_precision_multipliers)
}

pub fn get_virtual_price(swap: &Swap) -> u128
{
    let d = get_d(&_xp2(swap), get_a_precise_internal(swap));
    let supply : U256 = runtime::call_contract(
        ContractHash::new(swap.lp_token.into_hash().unwrap()),
        "total_supply",
        runtime_args! {},
    );
    if supply.as_u128() > 0 {
        return mul_div(d, (10u128).pow(POOL_PRECISION_DECIMALS as u32), (supply.as_u128()))
    }
    return 0;
}

/**
    * @notice Calculate the new balances of the tokens given the indexes of the token
    * that is swapped from (FROM) and the token that is swapped to (TO).
    * This function is used as a helper function to calculate how much TO token
    * the user should receive on swap.
    *
    * @param preciseA precise form of amplification coefficient
    * @param tokenIndexFrom index of FROM token
    * @param tokenIndexTo index of TO token
    * @param x the new total amount of FROM token
    * @param xp balances of the tokens in the pool
    * @return the amount of TO token that should remain in the pool
    */
fn get_y(
    precise_a: u128,
    token_index_from: usize,
    token_index_to: usize,
    x: u128,
    xp: &Vec<u128>
) -> u128 {
    let num_tokens = xp.len();
    require(
        token_index_from != token_index_to,
        Error::CantCompareTokenToItself
    );
    require(
        token_index_from < num_tokens && token_index_to < num_tokens,
        Error::TokenMustInPool
    );
    let num_tokens = num_tokens as u128;

    let d = get_d(&xp, precise_a);
    let mut c = d;
    let mut s = 0u128;
    let na = num_tokens * precise_a;

    let mut _x;
    for i in 0..(num_tokens as usize) {
        if i == token_index_from {
            _x = x;
        } else if i != token_index_to {
            _x = xp[i];
        } else {
            continue;
        }
        s = s + _x;
        c = mul_div(c, d, (_x * num_tokens));
        // If we were to protect the division loss we would have to keep the denominator separate
        // and divide at the end. However this leads to overflow with large numTokens or/and D.
        // c = c * D * D * D * ... overflow!
    }
    c = mul_div(c, d * ampl::A_PRECISION, (na * num_tokens));
    let b = s + mul_div(d, (ampl::A_PRECISION), na);

    let mut y_prev;
    let mut y = d;

    // iterative approximation
    for _i in 0..MAX_LOOP_LIMIT {
        y_prev = y;
        y = ((U256::from(y) * U256::from(y) + U256::from(c)) / U256::from(y * 2 + b - d)).as_u128();
        if math_utils::within1(y, y_prev) {
            return y;
        }
    }
    runtime::revert(Error::TokenIndexOutOfRange);
}

/**
    * @notice Externally calculates a swap between two tokens.
    * @param self Swap struct to read from
    * @param tokenIndexFrom the token to sell
    * @param tokenIndexTo the token to buy
    * @param dx the number of tokens to sell. If the token charges a fee on transfers,
    * use the amount that gets transferred after the fee.
    * @return dy the number of tokens the user will get
    */
pub fn calculate_swap(
    swap: &Swap,
    token_index_from: usize,
    token_index_to: usize,
    dx: u128,
) -> u128 {
    let (dy, _) = _calculate_swap(
        swap,
        token_index_from,
        token_index_to,
        dx,
        &swap.balances.clone()
    );
    dy
}

/**
    * @notice Internally calculates a swap between two tokens.
    *
    * @dev The caller is expected to transfer the actual amounts (dx and dy)
    * using the token contracts.
    *
    * @param self Swap struct to read from
    * @param tokenIndexFrom the token to sell
    * @param tokenIndexTo the token to buy
    * @param dx the number of tokens to sell. If the token charges a fee on transfers,
    * use the amount that gets transferred after the fee.
    * @return dy the number of tokens the user will get
    * @return dyFee the associated fee
    */
fn _calculate_swap(
    swap: &Swap,
    token_index_from: usize,
    token_index_to: usize,
    dx: u128,
    balances: &Vec<u128>
) -> (u128, u128) {
    let multipliers = swap.token_precision_multipliers.clone();
    let xp = _xp(&balances, &multipliers);
    require(
        token_index_from < xp.len() && token_index_to < xp.len(),
        Error::TokenIndexOutOfRange
    );
    let x = dx * multipliers[token_index_from] + xp[token_index_from];
    let y = get_y(
        get_a_precise_internal(swap),
        token_index_from,
        token_index_to,
        x,
        &xp
    );
    let mut dy = xp[token_index_to] - y - 1;
    let dy_fee = mul_div(dy, (swap.swap_fee as u128), FEE_DENOMINATOR);
    dy = (dy - dy_fee) / multipliers[token_index_to];
    (dy, dy_fee)
}

/**
    * @notice A simple method to calculate amount of each underlying
    * tokens that is returned upon burning given amount of
    * LP tokens
    *
    * @param amount the amount of LP tokens that would to be burned on
    * withdrawal
    * @return array of amounts of tokens user will receive
    */
pub fn calculate_remove_liquidity(swap: &mut Swap, amount: u128) -> Vec<u128>
{
    let total_supply : U256 = runtime::call_contract(
        ContractHash::new(swap.lp_token.into_hash().unwrap()),
        "total_supply",
        runtime_args! {},
    );
    _calculate_remove_liquidity(
        &swap.balances,
        amount,
        total_supply.as_u128()
    )
}

fn _calculate_remove_liquidity(
    balances: &Vec<u128>,
    amount: u128,
    total_supply: u128
) -> Vec<u128> {
    require(amount <= total_supply, Error::ExceedSupply);

    let mut amounts = vec![0; balances.len()];

    for i in 0..balances.len() {
        amounts[i] = balances[i] * amount / total_supply;
    }
    amounts
}

/**
    * @notice A simple method to calculate prices from deposits or
    * withdrawals, excluding fees but including slippage. This is
    * helpful as an input into the various "min" parameters on calls
    * to fight front-running
    *
    * @dev This shouldn't be used outside frontends for user estimates.
    *
    * @param self Swap struct to read from
    * @param amounts an array of token amounts to deposit or withdrawal,
    * corresponding to pooledTokens. The amount should be in each
    * pooled token's native precision. If a token charges a fee on transfers,
    * use the amount that gets transferred after the fee.
    * @param deposit whether this is a deposit or a withdrawal
    * @return if deposit was true, total amount of lp token that will be minted and if
    * deposit was false, total amount of lp token that will be burned
    */
pub fn calculate_token_amount(
    swap: &Swap,
    amounts: &Vec<u128>,
    deposit: bool
) -> u128 {
    let a = get_a_precise_internal(swap);
    let mut balances = swap.balances.clone();
    let multipliers = swap.token_precision_multipliers.clone();

    let d0 = get_d(&_xp(&balances, &multipliers), a);
    for i in 0..balances.len() {
        if deposit {
            balances[i] = balances[i] + amounts[i];
        } else {
            require(balances[i] <= amounts[i], Error::WithdrawTooMuch);
            balances[i] = balances[i] - amounts[i];
        }
    }
    let d1 = get_d(&_xp(&balances, &multipliers), a);

    let total_supply = erc20_helpers::get_total_supply(swap.lp_token);

    if deposit {
        return mul_div(d1 -d0, total_supply, d0);
    } else {
        return mul_div(d0 -d1, total_supply, d0);
    }
}

/**
    * @notice return accumulated amount of admin fees of the token with given index
    * @param self Swap struct to read from
    * @param index Index of the pooled token
    * @return admin balance in the token's precision
    */
pub fn get_admin_balance(swap: &Swap, index: usize) -> u128
{
    require(index < swap.pooled_tokens.len(), Error::TokenIndexOutOfRange);
    let balance_of_contract = erc20_helpers::get_balance(swap.pooled_tokens[index], get_self_key());
    balance_of_contract - swap.balances[index]
}

/**
* @notice internal helper function to calculate fee per token multiplier used in
* swap fee calculations
* @param swapFee swap fee for the tokens
* @param numTokens number of tokens pooled
*/
fn _fee_per_token(swap_fee: u128, num_tokens: u128) -> u128
{
    mul_div(swap_fee, (num_tokens as u128), (((num_tokens -1) * 4) as u128))
}

pub fn swap(
    swap: &mut Swap,
    token_index_from: usize,
    token_index_to: usize,
    dx_: u128,
    min_dy: u128
) -> u128 {
    let dx;
    {
        let balance_of_from = erc20_helpers::get_balance(swap.pooled_tokens[token_index_from].clone(), helpers::get_immediate_caller_key());
        require(
            dx_ <= balance_of_from,
            Error::CannotSwapMoreThanYouHave
        );
        // Transfer tokens first to see if a fee was charged on transfer
        let before_balance = erc20_helpers::get_balance(swap.pooled_tokens[token_index_from].clone(), helpers::get_self_key());
        
        erc20_helpers::transfer_from(swap.pooled_tokens[token_index_from], helpers::get_immediate_caller_key(), helpers::get_self_key(), dx_);

        let after_balance = erc20_helpers::get_balance(swap.pooled_tokens[token_index_from].clone(), helpers::get_self_key());

        // Use the actual transferred amount for AMM math
        dx = after_balance - before_balance;
    }
    let balances = swap.balances.clone();
    let (dy, dy_fee) = _calculate_swap(
        swap,
        token_index_from,
        token_index_to,
        dx,
        &balances
    );
    require(dy >= min_dy, Error::SwapNotResultInMinToken);

    let dy_admin_fee = mul_div(dy_fee, (swap.admin_fee as u128),FEE_DENOMINATOR) / (
        swap.token_precision_multipliers[token_index_to]
    );

    swap.balances[token_index_from] = swap.balances[token_index_from] + dx;
    swap.balances[token_index_to] = swap.balances[token_index_to] - dy - dy_admin_fee;
    
    erc20_helpers::transfer(swap.pooled_tokens[token_index_to], helpers::get_immediate_caller_key(), dy);

    events::emit(&CAFIDexEvent::TokenSwap {
        buyer: helpers::get_immediate_caller_key(),
        tokens_sold: dx,
        tokens_bought: dy,
        sold_id: token_index_from as u128,
        bought_id: token_index_to as u128
    });
    dy
}

pub fn add_liquidity(
    swap: &mut Swap,
    amounts_: Vec<u128>,
    min_to_mint: u128
) -> u128 {
    let mut amounts = amounts_.clone();
    let pooled_tokens = swap.pooled_tokens.clone();
    require(
        amounts.len() == pooled_tokens.len(),
        Error::AmountsNotMatchPooledTokens
    );

    // current state
    let mut v = ManageLiquidityInfo{
        d0: 0,
        d1: 0,
        d2: 0,
        precise_a: get_a_precise_internal(swap),
        lp_token: swap.lp_token,
        total_supply: 0,
        balances: swap.balances.clone(),
        multipliers: swap.token_precision_multipliers.clone()
    };
    v.total_supply = erc20_helpers::get_total_supply(v.lp_token);

    if v.total_supply != 0 {
        v.d0 = get_d(&_xp(&v.balances, &v.multipliers), v.precise_a);
    }

    let mut new_balances: Vec<u128> = vec![0; pooled_tokens.len()];

    for i in 0..pooled_tokens.len() {
        require(
            v.total_supply != 0 || amounts[i] > 0,
            Error::MustSupplyAllTokensInPool
        );

        // Transfer tokens first to see if a fee was charged on transfer
        if amounts[i] != 0 {
            let before_balance = erc20_helpers::get_balance(pooled_tokens[i].clone(), get_self_key());
            erc20_helpers::transfer_from(pooled_tokens[i], helpers::get_immediate_caller_key(), helpers::get_self_key(), amounts[i]);
            // Update the amounts[] with actual transfer amount
            amounts[i] = erc20_helpers::get_balance(pooled_tokens[i].clone(), get_self_key()) - before_balance;
        }

        new_balances[i] = v.balances[i] + amounts[i];
    }
    // invariant after change
    v.d1 = get_d(&_xp(&new_balances, &v.multipliers), v.precise_a);
    require(v.d1 > v.d0, Error::DShouldIncrease);

    // updated to reflect fees and calculate the user's LP tokens
    v.d2 = v.d1;
    let mut fees: Vec<u128> = vec![0; pooled_tokens.len()];

    if v.total_supply != 0 {
        let fee_per_token = _fee_per_token(
            swap.swap_fee as u128,
            pooled_tokens.len() as u128
        );
        for i in 0..pooled_tokens.len() {
            let ideal_balance = mul_div(v.d1, v.balances[i], v.d0);
            fees[i] = mul_div(fee_per_token, (math_utils::difference(ideal_balance, new_balances[i])), FEE_DENOMINATOR);
            swap.balances[i] = new_balances[i] - mul_div(fees[i], (swap.admin_fee as u128), FEE_DENOMINATOR);
            new_balances[i] = new_balances[i] - fees[i];
        }
        v.d2 = get_d(&_xp(&new_balances, &v.multipliers), v.precise_a);
    } else {
        // the initial depositor doesn't pay fees
        swap.balances = new_balances.clone();
    }

    let to_mint = if v.total_supply == 0 {
        v.d1
    } else {
        mul_div(v.d2 - v.d0, v.total_supply, v.d0)
    };

    require(to_mint >= min_to_mint, Error::CouldNotMintMinRequested);

    // mint the user's LP tokens
    let _: () = runtime::call_contract(
        ContractHash::new(v.lp_token.into_hash().unwrap()),
        "mint",
        runtime_args! {
            "owner" => helpers::get_immediate_caller_key(),
            "amount" => U256::from(to_mint)
        }
    );

    events::emit(&CAFIDexEvent::AddLiquidity {
        provider: helpers::get_immediate_caller_key(),
        token_amounts: amounts,
        fees: fees,
        invariant: v.d1,
        lp_token_supply: v.total_supply + to_mint
    });

    to_mint
}

pub fn remove_liquidity(
    swap: &mut Swap,
    amount: u128,
    min_amounts: Vec<u128>
) -> Vec<u128> {
    let lp_token = swap.lp_token;
    let pooled_tokens = swap.pooled_tokens.clone();
    require(amount <= erc20_helpers::get_balance(lp_token.clone(), helpers::get_immediate_caller_key()), Error::GreaterLPBalanceOf);
    require(
        min_amounts.len() == pooled_tokens.len(),
        Error::MinAmounsMustMatchPooledTokens
    );

    let balances = swap.balances.clone();
    let total_supply = erc20_helpers::get_total_supply(lp_token.clone());

    let amounts = _calculate_remove_liquidity(
        &balances,
        amount,
        total_supply
    );

    for i in 0..amounts.len() {
        require(amounts[i] >= min_amounts[i], Error::MinAmountsTooHigh);
        swap.balances[i] = balances[i] - amounts[i];

        erc20_helpers::transfer(pooled_tokens[i], helpers::get_immediate_caller_key(), amounts[i]);
    }

    let _: () = runtime::call_contract(
        ContractHash::new(lp_token.into_hash().unwrap()),
        "burn_from",
        runtime_args! {
            "address" => helpers::get_immediate_caller_key(),
            "amount" => U256::from(amount)
        }
    );

    events::emit(&CAFIDexEvent::RemoveLiquidity {
        provider: helpers::get_immediate_caller_key(),
        token_amounts: amounts.clone(),
        lp_token_supply: total_supply - amount
    });

    amounts
}

pub fn remove_liquidity_one_token(
    swap: &mut Swap,
    token_amount: u128,
    token_index: usize,
    min_amount: u128
) -> u128 {
    let lp_token = swap.lp_token.clone();
    let pooled_tokens = swap.pooled_tokens.clone();

    require(token_amount <= erc20_helpers::get_balance(lp_token.clone(), helpers::get_immediate_caller_key()), Error::GreaterLPBalanceOf);
    require(token_index < pooled_tokens.len(), Error::TokenNotFound);

    let total_supply = erc20_helpers::get_total_supply(lp_token.clone());

    let (dy, dy_fee) = _calculate_withdraw_one_token(
        swap,
        token_amount,
        token_index,
        total_supply
    );

    require(dy >= min_amount, Error::DYLessThanAmount);

    swap.balances[token_index] = swap.balances[token_index] - dy + mul_div(dy_fee, (swap.admin_fee as u128), FEE_DENOMINATOR);
    let _: () = runtime::call_contract(
        ContractHash::new(lp_token.into_hash().unwrap()),
        "burn_from",
        runtime_args! {
            "address" => helpers::get_immediate_caller_key(),
            "amount" => U256::from(token_amount)
        }
    );

    erc20_helpers::transfer(pooled_tokens[token_index], helpers::get_immediate_caller_key(), dy);

    events::emit(&CAFIDexEvent::RemoveLiquidityOne {
        provider: helpers::get_immediate_caller_key(),
        lp_token_amount: token_amount,
        lp_token_supply: total_supply,
        bought_id: token_index as u128,
        tokens_bought: dy
    });

    dy
}

pub fn remove_liquidity_imbalance(
    swap: &mut Swap,
    amounts: Vec<u128>,
    max_burn_amount: u128
) -> u128 {
    let mut v = ManageLiquidityInfo {
        d0: 0,
        d1: 0,
        d2: 0,
        precise_a: get_a_precise_internal(swap),
        lp_token: swap.lp_token.clone(),
        total_supply: 0,
        balances: swap.balances.clone(),
        multipliers: swap.token_precision_multipliers.clone()
    };
    v.total_supply = erc20_helpers::get_total_supply(v.lp_token.clone());

    let pooled_tokens = swap.pooled_tokens.clone();

    require(
        amounts.len() == pooled_tokens.len(),
        Error::AmountsNotMatchPooledTokens
    );

    require(
        max_burn_amount <= erc20_helpers::get_balance(v.lp_token.clone(), helpers::get_immediate_caller_key()) &&
        max_burn_amount != 0,
        Error::GreaterLPBalanceOf
    );

    let fee_per_token = _fee_per_token(swap.swap_fee as u128, pooled_tokens.len() as u128);
    let mut fees: Vec<u128> = vec![0; pooled_tokens.len()];
    {
        let mut balances1: Vec<u128> = vec![0; pooled_tokens.len()];
        v.d0 = get_d(&_xp(&v.balances, &v.multipliers), v.precise_a);
        for i in 0..pooled_tokens.len() {
            require(balances1[i] >= amounts[i], Error::GreaterLPBalanceOf);
            balances1[i] = v.balances[i] - amounts[i];
        }
        v.d1 = get_d(&_xp(&balances1, &v.multipliers), v.precise_a);

        for i in 0..pooled_tokens.len() {
            let ideal_balance = mul_div(v.d1, v.balances[i], v.d0);
            let difference = math_utils::difference(ideal_balance, balances1[i]);
            fees[i] = mul_div(fee_per_token, difference, FEE_DENOMINATOR);
            swap.balances[i] = balances1[i] - mul_div(fees[i], (swap.admin_fee as u128), FEE_DENOMINATOR);
            balances1[i] = balances1[i] - fees[i];
        }

        v.d2 = get_d(&_xp(&balances1, &v.multipliers), v.precise_a);
    }
    let mut token_amount = mul_div(v.d0 - v.d2, v.total_supply, v.d0);
    require(token_amount != 0, Error::BurnAmountCannotZero);
    token_amount = token_amount  + 1;

    require(token_amount <= max_burn_amount, Error::TokenAmountGreaterThanMaxBurn);

    let _: () = runtime::call_contract(
        ContractHash::new(v.lp_token.into_hash().unwrap()),
        "burn_from",
        runtime_args! {
            "address" => helpers::get_immediate_caller_key(),
            "amount" => U256::from(token_amount)
        }
    );

    for i in 0..pooled_tokens.len() {
        erc20_helpers::transfer(pooled_tokens[i], helpers::get_immediate_caller_key(), amounts[i]);
    }

    events::emit(&CAFIDexEvent::RemoveLiquidityImbalance {
        provider: helpers::get_immediate_caller_key(),
        token_amounts: amounts,
        fees: fees,
        invariant: v.d1,
        lp_token_supply: v.total_supply - token_amount
    });

    token_amount
}

pub fn withdraw_admin_fees(swap: &mut Swap, to: Key) {
    let pooled_tokens = swap.pooled_tokens.clone();
    for i in 0..pooled_tokens.len() {
        let token = pooled_tokens[i].clone();
        let balance = erc20_helpers::get_balance(token.clone(), helpers::get_self_key()) - swap.balances[i];
        if balance != 0 {
            erc20_helpers::transfer(token, to, balance);
        }
    }
}

pub fn set_admin_fee(swap: &mut Swap, new_admin_fee: u64) {
    require(new_admin_fee as u128 <= MAX_ADMIN_FEE, Error::FeeTooHigh);
    swap.admin_fee = new_admin_fee;
}

pub fn set_swap_fee(swap: &mut Swap, new_swap_fee: u64) {
    require(new_swap_fee as u128 <= MAX_SWAP_FEE, Error::FeeTooHigh);
    swap.swap_fee = new_swap_fee;
}