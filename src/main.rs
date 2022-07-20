use core::time;
use secp256k1::SecretKey;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;
use web3::api::{self, Namespace};
use web3::types::{H256, U256};
use web3::{
    self,
    contract::{Contract, Options},
    types::H160,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "platform tool")]
struct Opt {
    // /// version
    #[structopt(name = "version", short = "v")]
    pub version: bool,

    // Note that we mark a field as a subcommand
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "sub command")]
enum Command {
    Account(Account),
    Contract(Contr),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "account manage")]
enum Account {
    Getbalance(GetBalance),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract manage")]
enum Contr {
    Deploy(Deploy),
    Call(Call),
    Query(Query),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract deploy")]
struct Deploy {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpcurl")]
    pub rpcurl: String,

    /// contract code
    #[structopt(
        name = "contract byte code path",
        short = "c",
        long = "code",
        parse(from_os_str)
    )]
    pub code: PathBuf,

    /// contract abi
    #[structopt(
        name = "contract abi path",
        short = "a",
        long = "abi",
        parse(from_os_str)
    )]
    pub abi: PathBuf,

    /// secret key
    #[structopt(name = "secret key", short = "k", long = "sec_key")]
    pub sec_key: String,

    /// gas
    #[structopt(name = "gas", short = "g", long)]
    pub gas: Option<i32>,

    /// gas price
    #[structopt(name = "gas price", short = "p", long = "gas_price")]
    pub gas_price: Option<i32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract call")]
struct Call {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpcurl")]
    pub rpcurl: String,

    /// contract addr
    #[structopt(name = "contract addr", short = "t", long = "contr_addr")]
    pub contr_addr: String,

    /// contract abi
    #[structopt(
        name = "contract abi path",
        short = "a",
        long = "abi",
        parse(from_os_str)
    )]
    pub abi: PathBuf,

    /// secret key
    #[structopt(name = "secret key", short = "k", long = "sec_key")]
    pub sec_key: String,

    /// gas
    #[structopt(name = "gas", short = "g", long)]
    pub gas: Option<i32>,

    /// gas price
    #[structopt(name = "gas price", short = "p", long = "gas_price")]
    pub gas_price: Option<i32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract query")]
struct Query {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpcurl")]
    pub rpcurl: String,

    /// contract addr
    #[structopt(name = "contract addr", short = "t", long = "contr_addr")]
    pub contr_addr: String,

    /// contract abi
    #[structopt(
        name = "contract abi path",
        short = "a",
        long = "abi",
        parse(from_os_str)
    )]
    pub abi: PathBuf,

    /// gas
    #[structopt(name = "gas", short = "g", long)]
    pub gas: Option<i32>,

    /// gas price
    #[structopt(name = "gas price", short = "p", long = "gas_price")]
    pub gas_price: Option<i32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "get balance")]
struct GetBalance {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpcurl")]
    pub rpcurl: String,

    #[structopt(name = "account")]
    pub account: String,
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);

    match opt.cmd {
        Some(cmd) => match cmd {
            Command::Account(account) => match account {
                Account::Getbalance(getbalance) => {
                    let balance = get_balance(&getbalance.rpcurl, &getbalance.account).await?;
                    println!("account balance {:?}: {}", getbalance.account, balance);
                }
            },
            Command::Contract(contract) => match contract {
                Contr::Deploy(deploy) => {
                    let transaction_hash = contract_deploy(
                        &deploy.rpcurl,
                        &deploy.sec_key,
                        &deploy.code,
                        &deploy.abi,
                        deploy.gas,
                        deploy.gas_price,
                    )
                    .await?;
                    println!("contract address: {:?}", transaction_hash);
                }
                Contr::Call(call) => {
                    let transaction_hash = contract_call(
                        &call.rpcurl,
                        &call.contr_addr,
                        &call.sec_key,
                        &call.abi,
                        call.gas,
                        call.gas_price,
                    )
                    .await?;
                    println!("transaction hash: {:?}", transaction_hash);
                }
                Contr::Query(query) => {
                    let result = contract_query(
                        &query.rpcurl,
                        &query.contr_addr,
                        &query.abi,
                        query.gas,
                        query.gas_price,
                    )
                    .await?;
                    println!("query result: {:?}", result);
                }
            },
        },
        None => {
            if opt.version {
                println!("version 0.0.1");
            } else {
                println!("please input correct argument or subcommand!");
            }
        }
    }

    Ok(())
}

async fn contract_deploy(
    rpcurl: &str,
    // _account: &str,
    sec_key: &str,
    bytecode_path: &Path,
    abi_path: &Path,
    gas: Option<i32>,
    gas_price: Option<i32>,
) -> web3::contract::Result<H160> {
    let transport = web3::transports::Http::new(rpcurl)?;
    let web3 = web3::Web3::new(transport);
    // let _account: H160 = _account.parse().unwrap();

    let byetcode = fs::read(bytecode_path).unwrap();
    let abi = fs::read(abi_path).unwrap();

    let secretkey = SecretKey::from_str(sec_key).unwrap();

    let contract = Contract::deploy(web3.eth(), &abi)?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| {
            if let Some(gas) = gas {
                opt.gas = Some(gas.into());
            }
            if let Some(gas_price) = gas_price {
                opt.gas_price = Some(gas_price.into());
            }
        }))
        .sign_with_key_and_execute(
            std::str::from_utf8(&byetcode).unwrap(),
            (),
            &secretkey,
            None,
        )
        .await?;

    Ok(contract.address())
}

async fn contract_call(
    rpcurl: &str,
    contr_addr: &str,
    sec_key: &str,
    // _account: &str,
    abi_path: &Path,
    gas: Option<i32>,
    gas_price: Option<i32>,
) -> web3::contract::Result<H256> {
    let transport = web3::transports::Http::new(rpcurl)?;
    let eth = api::Eth::new(transport);
    let abi = fs::read(abi_path).unwrap();
    let contr_addr: H160 = contr_addr.parse().unwrap();
    // let _account: H160 = _account.parse().unwrap();
    let contract = Contract::from_json(eth, contr_addr, &abi)?;
    let secretkey = SecretKey::from_str(sec_key).unwrap();
    let mut opt = Options::default();
    if let Some(gas) = gas {
        opt.gas = Some(gas.into());
    }
    if let Some(gas_price) = gas_price {
        opt.gas_price = Some(gas_price.into());
    }

    let transaction_hash = contract
        .signed_call("store", (12345u32,), opt, &secretkey)
        .await?;

    Ok(transaction_hash)
}

async fn contract_query(
    rpcurl: &str,
    contr_addr: &str,
    // _account: &str,
    abi_path: &Path,
    gas: Option<i32>,
    gas_price: Option<i32>,
) -> web3::contract::Result<U256> {
    let transport = web3::transports::Http::new(rpcurl)?;
    let eth = api::Eth::new(transport);
    let abi = fs::read(abi_path).unwrap();
    let contr_addr: H160 = contr_addr.parse().unwrap();
    // let _account: H160 = _account.parse().unwrap();
    let contract = Contract::from_json(eth, contr_addr, &abi)?;
    // let _secretkey = SecretKey::from_str(_sec_key).unwrap();
    let mut opt = Options::default();
    if let Some(gas) = gas {
        opt.gas = Some(gas.into());
    }
    if let Some(gas_price) = gas_price {
        opt.gas_price = Some(gas_price.into());
    }

    let result = contract.query("retrieve", (), None, opt, None).await?;
    Ok(result)
}

async fn get_balance(rpcurl: &str, account: &str) -> web3::Result<U256> {
    let transport = web3::transports::Http::new(&rpcurl)?;
    let web3 = web3::Web3::new(transport);

    let account: H160 = account.parse().unwrap();
    let balance = web3.eth().balance(account, None).await?;

    Ok(balance)
}
