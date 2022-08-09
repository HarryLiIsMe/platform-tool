use secp256k1::SecretKey;
use std::{fs, str::FromStr, time};
use web3::{
    self,
    api::{self, Namespace},
    contract::{Contract, Options},
    ethabi::Token,
    types::H160,
    types::{H256, U256},
};

const PULL_INTERVAL: u64 = 50;

pub(crate) async fn contract_deploy(
    rpc_url: &str,
    sec_key: &str,
    code_path: &str,
    abi_path: &str,
    gas: u32,
    gas_price: u32,
    args: Vec<Token>,
) -> web3::contract::Result<H160> {
    let transport = web3::transports::Http::new(rpc_url)?;
    let web3 = web3::Web3::new(transport);

    // let _account: H160 = _account.parse().unwrap();

    let eth = web3.eth();
    // let nonce = eth
    //     .transaction_count(account.parse().unwrap(), None)
    //     .await?;
    // println!("nonce: {}, nonce_add: {}", nonce, nonce_add);

    let byetcode = fs::read(code_path).unwrap();
    let abi = fs::read(abi_path).unwrap();

    let secretkey = SecretKey::from_str(sec_key).unwrap();

    let contract;
    if args.is_empty() {
        contract = Contract::deploy(eth, &abi)?
            .confirmations(1)
            .poll_interval(time::Duration::from_millis(PULL_INTERVAL))
            .options(Options::with(|opt| {
                opt.gas = Some(gas.into());
                opt.gas_price = Some(gas_price.into());
                // opt.nonce = Some(nonce + nonce_add);
            }))
            .sign_with_key_and_execute(
                std::str::from_utf8(&byetcode).unwrap(),
                (),
                &secretkey,
                None,
            )
            .await?;
    } else {
        contract = Contract::deploy(eth, &abi)?
            .confirmations(1)
            .poll_interval(time::Duration::from_millis(PULL_INTERVAL))
            .options(Options::with(|opt| {
                opt.gas = Some(gas.into());
                opt.gas_price = Some(gas_price.into());
                // opt.nonce = Some(nonce + nonce_add);
            }))
            .sign_with_key_and_execute(
                std::str::from_utf8(&byetcode).unwrap(),
                args,
                &secretkey,
                None,
            )
            .await?;
    }

    Ok(contract.address())
}

pub(crate) async fn contract_call(
    rpc_url: &str,
    contr_addr: &str,
    sec_key: &str,
    // _account: &str,
    abi_path: &str,
    gas: u32,
    gas_price: u32,
    func_name: &str,
    args: Vec<Token>,
) -> web3::contract::Result<H256> {
    let transport = web3::transports::Http::new(rpc_url)?;
    let eth = api::Eth::new(transport);
    let abi = fs::read(abi_path).unwrap();
    let contr_addr: H160 = contr_addr.parse().unwrap();
    // let _account: H160 = _account.parse().unwrap();
    let contract = Contract::from_json(eth, contr_addr, &abi)?;
    let secretkey = SecretKey::from_str(sec_key).unwrap();

    let mut opt = Options::default();
    opt.gas = Some(gas.into());
    opt.gas_price = Some(gas_price.into());

    let transaction_hash;
    if args.is_empty() {
        transaction_hash = contract.signed_call(func_name, (), opt, &secretkey).await?;
    } else {
        transaction_hash = contract
            .signed_call(func_name, args, opt, &secretkey)
            .await?;
    }

    Ok(transaction_hash)
}

pub(crate) async fn contract_query(
    rpc_url: &str,
    contr_addr: &str,
    // _account: &str,
    abi_path: &str,
    func_name: &str,
    args: Vec<Token>,
) -> web3::contract::Result<U256> {
    let transport = web3::transports::Http::new(rpc_url)?;
    let eth = api::Eth::new(transport);
    let abi = fs::read(abi_path).unwrap();
    let contr_addr: H160 = contr_addr.parse().unwrap();
    // let _account: H160 = _account.parse().unwrap();
    let contract = Contract::from_json(eth, contr_addr, &abi)?;
    // let _secretkey = SecretKey::from_str(_sec_key).unwrap();
    let opt = Options::default();

    let result;
    if args.is_empty() {
        result = contract.query(func_name, (), None, opt, None).await?;
    } else {
        result = contract.query(func_name, args, None, opt, None).await?;
    }

    Ok(result)
}

pub(crate) async fn get_balance(rpc_url: &str, account: &str) -> web3::Result<U256> {
    let transport = web3::transports::Http::new(&rpc_url)?;
    let web3 = web3::Web3::new(transport);

    let account: H160 = account.parse().unwrap();
    let balance = web3.eth().balance(account, None).await?;

    Ok(balance)
}
