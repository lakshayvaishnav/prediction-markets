pub use anchor_lang::prelude::*;

// bancor's formaula implementation

// TODO :  add the logic for fees here only.
pub fn calculate_purchase_return(
    connector_weight: u32,
    virtual_sol_reserve: u64,
    virtual_token_reserve: u64,
    deposit_amount: u64
) -> u64 {
    let cw = (connector_weight as f64) / 100.0;
    let virtual_sol = virtual_sol_reserve as f64;
    let virtual_token = virtual_token_reserve as f64;

    // if you charge a fee, apply it here:
    let amount_after_fee = deposit_amount as f64;

    let base = 1.0 + amount_after_fee / virtual_sol;
    let tokens_out_f = virtual_token * (base.powf(cw) - 1.0);
    tokens_out_f.floor() as u64
}

pub fn calculate_sale_return(
    connector_weight: u32,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    sell_tokens: u64
) -> u64 {
    let cw = (connector_weight as f64) / 100.0;
    let virtual_sol = virtual_sol_reserves as f64;
    let virtual_token = virtual_token_reserves as f64;

    // apply any sell fees here
    let amount_after_fee = sell_tokens as f64;

    let base = 1.0 - amount_after_fee / virtual_token;
    let sol_out_f = virtual_sol * (1.0 - base.powf(1.0 / cw));
    sol_out_f.floor() as u64
}