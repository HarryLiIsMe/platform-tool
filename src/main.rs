use structopt::StructOpt;

mod commands;
mod multi_tasks;
mod task_impl;
use commands::{
    parse_call_json, parse_deploy_json, parse_query_json, Account, Command, Contr, Opt,
};
use multi_tasks::{concurrent_contract_call, concurrent_contract_deploy};
use task_impl::{contract_query, get_balance};

// const MIN_TASK: u32 = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);

    match opt.cmd {
        Some(cmd) => match cmd {
            Command::Account(account) => match account {
                Account::Getbalance(getbalance) => {
                    let balance = get_balance(&getbalance.rpc_url, &getbalance.account).await?;
                    println!("account balance {:?}: {}", getbalance.account, balance);
                }
            },
            Command::Contract(contract) => match contract {
                Contr::Deploy(deploy) => {
                    let deploy_json = parse_deploy_json(&deploy.config).await?;

                    // let count = deploy.count;
                    // let total_task;
                    // if count.is_none() {
                    //     total_task = 1;
                    // } else {
                    //     total_task = count.unwrap();
                    //     if total_task < MIN_TASK {
                    //         panic!("total task not be less than {}", MIN_TASK);
                    //     }
                    // }
                    let (success_task, total_times) =
                        concurrent_contract_deploy(&deploy.rpc_url, deploy_json).await?;
                    println!(
                        "success task: {} total times: {} average time: {}",
                        success_task,
                        total_times,
                        if success_task == 0 {
                            0
                        } else {
                            total_times / success_task as u128
                        }
                    );
                }
                Contr::Call(call) => {
                    let call_json = parse_call_json(call.config).await?;

                    let (success_task, total_times) =
                        concurrent_contract_call(&call.rpc_url, call_json).await?;
                    println!(
                        "success task: {} total times: {} average time: {}",
                        success_task,
                        total_times,
                        if success_task == 0 {
                            0
                        } else {
                            total_times / success_task as u128
                        }
                    );
                }
                Contr::Query(query) => {
                    let query_json_obj = parse_query_json(query.config).await?;
                    let result = contract_query(
                        &query.rpc_url,
                        &query_json_obj.contract_addr,
                        &query_json_obj.abi_path,
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
