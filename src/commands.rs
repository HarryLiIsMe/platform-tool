use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs;
use web3::{
    contract::tokens::Tokenizable,
    ethabi::{Int, Token, Uint},
    types::{Address, H160, H256, U128, U256},
};

#[derive(StructOpt, Debug)]
#[structopt(name = "platform tool")]
pub(crate) struct Opt {
    // /// version
    #[structopt(name = "version", short = "v")]
    pub(crate) version: bool,

    // Note that we mark a field as a subcommand
    #[structopt(subcommand)]
    pub(crate) cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "sub command")]
pub(crate) enum Command {
    Account(Account),
    Contract(Contr),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "account manage")]
pub(crate) enum Account {
    Getbalance(GetBalance),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract manage")]
pub(crate) enum Contr {
    Deploy(Deploy),
    Call(Call),
    Query(Query),
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "contract deploy")]
pub(crate) struct Deploy {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub(crate) rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub(crate) config: PathBuf,

    // /// total execute count you need
    // #[structopt(name = "execute count", short = "c", long = "count")]
    // pub(crate) count: Option<u32>,
    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub(crate) _max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract call")]
pub(crate) struct Call {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub(crate) rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub(crate) config: PathBuf,

    // /// total execute count you need
    // #[structopt(name = "execute count", short = "c", long = "count")]
    // pub(crate) count: Option<u32>,
    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub(crate) _max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract query")]
pub(crate) struct Query {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub(crate) rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub(crate) config: PathBuf,

    /// total execute count you need
    #[structopt(name = "execute count", short = "c", long = "count")]
    pub(crate) _count: Option<u32>,

    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub(crate) _max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "get balance")]
pub(crate) struct GetBalance {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub(crate) rpc_url: String,

    #[structopt(name = "account")]
    pub(crate) account: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct DeployJsonObj {
    pub(crate) code_path: String,
    pub(crate) abi_path: String,
    pub(crate) sec_key: String,
    pub(crate) gas: u32,
    pub(crate) gas_price: u32,
    pub(crate) args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CallJsonObj {
    pub(crate) contract_addr: String,
    pub(crate) abi_path: String,
    pub(crate) sec_key: String,
    pub(crate) gas: u32,
    pub(crate) gas_price: u32,
    pub(crate) func_name: String,
    pub(crate) args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct DeployJson {
    pub(crate) deploy_obj: Vec<DeployJsonObj>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CallJson {
    pub(crate) call_obj: Vec<CallJsonObj>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct QueryJson {
    pub(crate) contract_addr: String,
    pub(crate) abi_path: String,
    pub(crate) func_name: String,
    pub(crate) args: String,
}

pub(crate) async fn parse_deploy_json(pat: &PathBuf) -> anyhow::Result<DeployJson> {
    let deploy_json_bytes = fs::read(pat).await?;
    let deply_json_obj: DeployJson = serde_json::from_slice(deploy_json_bytes.as_slice())?;

    return Ok(deply_json_obj);
}

pub(crate) async fn parse_call_json(pat: PathBuf) -> anyhow::Result<CallJson> {
    let call_json_bytes = fs::read(pat).await?;
    let call_json_obj: CallJson = serde_json::from_slice(call_json_bytes.as_slice())?;

    return Ok(call_json_obj);
}

pub(crate) async fn parse_query_json(pat: PathBuf) -> anyhow::Result<QueryJson> {
    let query_json_bytes = fs::read(pat).await?;
    let query_json_obj: QueryJson = serde_json::from_slice(query_json_bytes.as_slice())?;

    return Ok(query_json_obj);
}

pub(crate) fn parse_args_csv(args: &str) -> anyhow::Result<Vec<Token>> {
    let mut res: Vec<Token> = Vec::new();

    let args_str = args.to_string();
    let mut csv_reader1 = csv::Reader::from_reader(args_str.as_bytes());

    for args in csv_reader1.headers() {
        for arg in args {
            if arg == "" {
                bail!("arg format error!!!");
            } else if let Ok(arg_bool) = arg.parse::<bool>() {
                res.push(arg_bool.into_token());
            } else if let Ok(arg_int) = arg.parse::<Int>() {
                res.push(arg_int.into_token());
            } else if let Ok(arg_uint) = arg.parse::<Uint>() {
                res.push(arg_uint.into_token());
            } else if let Ok(arg_address) = arg.parse::<Address>() {
                res.push(arg_address.into_token());
            } else if let Ok(arg_h160) = arg.parse::<H160>() {
                res.push(arg_h160.into_token());
            } else if let Ok(arg_h256) = arg.parse::<H256>() {
                res.push(arg_h256.into_token());
            } else if let Ok(arg_u128) = arg.parse::<U128>() {
                res.push(arg_u128.into_token());
            } else if let Ok(arg_u256) = arg.parse::<U256>() {
                res.push(arg_u256.into_token());
            } else {
                let arg_string = arg.to_string();
                res.push(arg_string.into_token());
            }
        }
    }

    return Ok(res);
}
