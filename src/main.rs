use structopt::StructOpt;

mod commands;
mod multi_tasks;
mod task_impl;
use commands::{
    parse_call_json, parse_deploy_json, parse_query_json, Account, Command, Contr, Opt,
};
use commands::{CallJsonObj, DeployJsonObj};
use task_impl::{contract_query, get_balance};

use multi_tasks::multi_tasks_impl;
use task_impl::{contract_call, contract_deploy};
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

                    let mut vf = Vec::new();
                    for deploy_obj in deploy_json.deploy_obj {
                        let rpc_url = deploy.rpc_url.clone();

                        let DeployJsonObj {
                            code_path,
                            abi_path,
                            sec_key,
                            gas,
                            gas_price,
                            args: _args,
                        } = deploy_obj;

                        let f = move || async move {
                            match contract_deploy(
                                &rpc_url, &sec_key, &code_path, &abi_path, gas, gas_price,
                            )
                            .await
                            {
                                Ok(v) => {
                                    println!("contract address: {:?}", v);
                                    return Ok(());
                                }
                                Err(e) => {
                                    println!("deploy contract failed: {:?}", e);
                                    anyhow::bail!("deploy failed");
                                }
                            };
                        };

                        vf.push(f);
                    }

                    let (success_task, total_times) = multi_tasks_impl(vf).await?;
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

                    let mut vf = Vec::new();
                    for call_obj in call_json.call_obj {
                        let rpc_url = call.rpc_url.clone();

                        let CallJsonObj {
                            contract_addr,
                            abi_path,
                            sec_key,
                            gas,
                            gas_price,
                            args: _args,
                        } = call_obj;

                        let f = move || async move {
                            match contract_call(
                                &rpc_url,
                                &sec_key,
                                &contract_addr,
                                &abi_path,
                                gas,
                                gas_price,
                            )
                            .await
                            {
                                Ok(v) => {
                                    println!("transaction hash: {:?}", v);
                                    return Ok(());
                                }
                                Err(e) => {
                                    println!("call contract failed: {:?}", e);
                                    anyhow::bail!("call failed");
                                }
                            };
                        };

                        vf.push(f);
                    }

                    let (success_task, total_times) = multi_tasks_impl(vf).await?;

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
