#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw20_base::enumerable::{query_all_accounts, query_all_allowances};
use cw20_base::msg::{ExecuteMsg, QueryMsg};
use cw20_base::ContractError;

use crate::msg::MigrateMsg;
use cw2::set_contract_version;
// CW20 imported functions
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};

// CW20 imported functions
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, query_balance, query_download_logo, query_marketing_info, query_minter,
    query_token_info,
};

// Token name
const CONTRACT_NAME: &str = "crates.io:mide";
// Contract version
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: cw20_base::msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    cw20_base::contract::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, cw20_base::ContractError> {
    match msg {
        // Function to transfer the token
        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, env, info, recipient, amount)
        }
        // Function to burn or make the token disappear
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        // Function to send the token
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute_send(deps, env, info, contract, amount, msg),
        // Function to mint the token
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        // Function to decide the amout of token a spender can increase to
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        // Function to decide the amout of token a spender can decrease to
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        // Funtion that allows the token owner to transfer thier token
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        // Funtion that allows the token owner to mint thier token
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
        // Funtion that allows the token owner to send thier token
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => execute_send_from(deps, env, info, owner, contract, amount, msg),
        // Funtion for the token marketing info
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        // Function for the token logo
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
    }
}

// Function to make read requests only
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // unction to check a user balance
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        // Function to check the token info of a user
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        // Function to check who can mint a token
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        // Function to check how much a user is allowed to borrow
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        // Function to check how much all users are allowed to borrow
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_allowances(deps, owner, start_after, limit)?),
        // Function to check how many users are with the token
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        // Function to check the marketing info
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        // Function to download the logo
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

// A function that is supposed to migrate the token but does not.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
