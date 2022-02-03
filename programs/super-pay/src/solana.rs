use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub fn get_all_account() -> Vec<(Pubkey, Account)> {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);

    let pubkey = Pubkey::from_str("2F2XupZErDDHKniN4nkEHGsf7j7c7egmMHzVrhWpfXi2").unwrap();
    let res = client.get_program_accounts(&pubkey).unwrap();
    res
}
#[warn(dead_code)]
pub fn get_all_account_yeild() -> Vec<(Pubkey, Account)> {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);

    let pubkey = Pubkey::from_str("AZzGpBGZ2bXdh8AV2xhTw8g5HEGhkXteswh77LWXG7JJ").unwrap();
    let res = client.get_program_accounts(&pubkey).unwrap();
    res
}
#[warn(dead_code)]
pub fn get_rent_exemption_yield() -> u64 {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);
    client.get_minimum_balance_for_rent_exemption(170).unwrap()
}

pub fn get_rent_exemption() -> u64 {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);
    client.get_minimum_balance_for_rent_exemption(97).unwrap()
}
