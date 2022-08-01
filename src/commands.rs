use anyhow;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs;

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
pub(crate) struct DeployJson {
    pub(crate) deploy_obj: Vec<DeployJsonObj>,
}

pub(crate) async fn parse_deploy_json(pat: &PathBuf) -> anyhow::Result<DeployJson> {
    let deploy_json_bytes = fs::read(pat).await?;
    let deply_json_obj: DeployJson = serde_json::from_slice(deploy_json_bytes.as_slice())?;

    return Ok(deply_json_obj);
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CallJsonObj {
    pub(crate) contract_addr: String,
    pub(crate) abi_path: String,
    pub(crate) sec_key: String,
    pub(crate) gas: u32,
    pub(crate) gas_price: u32,
    pub(crate) args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CallJson {
    pub(crate) call_obj: Vec<CallJsonObj>,
}

pub(crate) async fn parse_call_json(pat: PathBuf) -> anyhow::Result<CallJson> {
    let call_json_bytes = fs::read(pat).await?;
    let call_json_obj: CallJson = serde_json::from_slice(call_json_bytes.as_slice())?;

    return Ok(call_json_obj);
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct QueryJson {
    pub(crate) contract_addr: String,
    pub(crate) abi_path: String,
    pub(crate) args: String,
}

pub(crate) async fn parse_query_json(pat: PathBuf) -> anyhow::Result<QueryJson> {
    let query_json_bytes = fs::read(pat).await?;
    let query_json_obj: QueryJson = serde_json::from_slice(query_json_bytes.as_slice())?;

    return Ok(query_json_obj);
}
