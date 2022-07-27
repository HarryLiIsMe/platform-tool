use anyhow;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs;

#[derive(StructOpt, Debug)]
#[structopt(name = "platform tool")]
pub struct Opt {
    // /// version
    #[structopt(name = "version", short = "v")]
    pub version: bool,

    // Note that we mark a field as a subcommand
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "sub command")]
pub enum Command {
    Account(Account),
    Contract(Contr),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "account manage")]
pub enum Account {
    Getbalance(GetBalance),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract manage")]
pub enum Contr {
    Deploy(Deploy),
    Call(Call),
    Query(Query),
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = "contract deploy")]
pub struct Deploy {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub config: PathBuf,

    // /// total execute count you need
    // #[structopt(name = "execute count", short = "c", long = "count")]
    // pub count: Option<u32>,
    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract call")]
pub struct Call {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub config: PathBuf,

    // /// total execute count you need
    // #[structopt(name = "execute count", short = "c", long = "count")]
    // pub count: Option<u32>,
    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "contract query")]
pub struct Query {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub rpc_url: String,

    /// config file path
    #[structopt(name = "config file", short = "g", long = "config", parse(from_os_str))]
    pub config: PathBuf,

    /// total execute count you need
    #[structopt(name = "execute count", short = "c", long = "count")]
    pub count: Option<u32>,

    /// max concurrent tasks
    #[structopt(name = "max concurrent tasks", short = "m", long = "max-multi")]
    pub max_concurrent: Option<u32>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "get balance")]
pub struct GetBalance {
    /// http rpc url
    #[structopt(name = "rpc url", short = "u", long = "rpc-url")]
    pub rpc_url: String,

    #[structopt(name = "account")]
    pub account: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeployJsonObj {
    pub code_path: String,
    pub abi_path: String,
    pub sec_key: String,
    pub gas: u32,
    pub gas_price: u32,
    pub args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeployJson {
    pub deploy_obj: Vec<DeployJsonObj>,
}

pub async fn parse_deploy_json(pat: &PathBuf) -> anyhow::Result<DeployJson> {
    let deploy_json_bytes = fs::read(pat).await?;
    let deply_json_obj: DeployJson = serde_json::from_slice(deploy_json_bytes.as_slice())?;

    return Ok(deply_json_obj);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallJsonObj {
    pub contract_addr: String,
    pub abi_path: String,
    pub sec_key: String,
    pub gas: u32,
    pub gas_price: u32,
    pub args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallJson {
    pub call_obj: Vec<CallJsonObj>,
}

pub async fn parse_call_json(pat: PathBuf) -> anyhow::Result<CallJson> {
    let call_json_bytes = fs::read(pat).await?;
    let call_json_obj: CallJson = serde_json::from_slice(call_json_bytes.as_slice())?;

    return Ok(call_json_obj);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryJson {
    pub contract_addr: String,
    pub abi_path: String,
    pub args: String,
}

pub async fn parse_query_json(pat: PathBuf) -> anyhow::Result<QueryJson> {
    let query_json_bytes = fs::read(pat).await?;
    let query_json_obj: QueryJson = serde_json::from_slice(query_json_bytes.as_slice())?;

    return Ok(query_json_obj);
}
