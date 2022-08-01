// use crate::get_timestamp;

use anyhow;
use lazy_static::lazy_static;
use tokio::sync::mpsc::Receiver;

use std::future::Future;

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub(crate) static ref CUR_TASKS: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    pub(crate) static ref MAX_TASKS: Arc<AtomicU32> = Arc::new(AtomicU32::new(2));
    // total success tasks、total tasks cost time、average tasks cost time queue
    pub(crate) static ref RES_QUEUE_SECS: Arc<Mutex<(u32, u128, Vec::<u128>)>> = Arc::new(Mutex::new((0, 0, Vec::new())));

}
const RES_QUEUE_MAX_LEN: usize = 10;
const UPDATE_INTERVAL: u64 = 300;
const DELTA_RANGE: u128 = 100;

pub(crate) async fn multi_tasks_impl<F, T>(vf: Vec<F>) -> anyhow::Result<(u32, u128)>
where
    F: FnOnce() -> T,
    T: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    let mut task_queue = Vec::with_capacity(vf.len());
    for f in vf {
        let af = f();
        let task = tokio::spawn(async move {
            CUR_TASKS.store(CUR_TASKS.load(Ordering::Acquire) + 1, Ordering::Release);

            let beg_time = get_timestamp();

            match af.await {
                Ok(_) => {
                    let end_time = get_timestamp();
                    update_res_queue_secs(end_time - beg_time).await;
                }
                Err(_) => {}
            };
            CUR_TASKS.store(CUR_TASKS.load(Ordering::Acquire) - 1, Ordering::Release);
        });
        task_queue.push(task);

        while MAX_TASKS.load(Ordering::Acquire) <= CUR_TASKS.load(Ordering::Acquire) {
            let task = task_queue.pop().unwrap();
            task.await?;
        }
    }

    let (tx1, rx1) = tokio::sync::mpsc::channel(2);
    tokio::spawn(max_tasks_update(rx1));

    for task in task_queue {
        task.await?;
    }

    tx1.send(()).await?;

    // error occur
    // return anyhow::Ok((RES_QUEUE_SECS.lock().await.0, RES_QUEUE_SECS.lock().await.1));

    let success_task = RES_QUEUE_SECS.lock().await.0;
    let total_times = RES_QUEUE_SECS.lock().await.1;

    return anyhow::Ok((success_task, total_times));
}

async fn max_tasks_update(mut rx: Receiver<()>) {
    loop {
        let res_queue_secs = RES_QUEUE_SECS.lock().await;
        let average_time_queue = &res_queue_secs.2;
        if average_time_queue.len() > 1 {
            let end_cost_time = average_time_queue.iter().last().unwrap();
            let mut big: u8 = 0;
            let mut less: u8 = 0;

            for cost in average_time_queue.iter().rev().skip(1) {
                if end_cost_time > cost && (end_cost_time - *cost) > DELTA_RANGE {
                    big += 1;
                }
                if end_cost_time < cost && (*cost - end_cost_time) > DELTA_RANGE {
                    less += 1;
                }
            }

            if big > less {
                MAX_TASKS.store(2 * MAX_TASKS.load(Ordering::Acquire), Ordering::Release);
            } else if big < less {
                MAX_TASKS.store(MAX_TASKS.load(Ordering::Acquire) - 1, Ordering::Release);
            } else {
                let end_cost_time2 = average_time_queue
                    .iter()
                    .rev()
                    .skip(1)
                    .rev()
                    .last()
                    .unwrap();

                if end_cost_time > end_cost_time2 && end_cost_time - end_cost_time2 > DELTA_RANGE {
                    MAX_TASKS.store(2 * MAX_TASKS.load(Ordering::Acquire), Ordering::Release);
                } else if end_cost_time < end_cost_time2
                    && end_cost_time2 - end_cost_time > DELTA_RANGE
                {
                    MAX_TASKS.store(MAX_TASKS.load(Ordering::Acquire) - 1, Ordering::Release);
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(UPDATE_INTERVAL)).await;
        if rx.try_recv().is_ok() {
            break;
        }
    }
}

async fn update_res_queue_secs(interval: u128) {
    let mut res_queue_secs = RES_QUEUE_SECS.lock().await;

    res_queue_secs.0 += 1;
    res_queue_secs.1 += interval;
    let aveage_time = res_queue_secs.1 / res_queue_secs.0 as u128;
    res_queue_secs.2.push(aveage_time);

    while res_queue_secs.2.len() > RES_QUEUE_MAX_LEN {
        res_queue_secs.2.pop();
    }
}

fn get_timestamp() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
