use std::env;
use async_std::task;
use futures::future::join_all;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        tasks.push(task::sleep(Duration::from_secs(10)));
    }

    join_all(tasks).await;
}
