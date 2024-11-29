use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        tasks.push(sleep(Duration::from_secs(10)));
    }
    futures::future::join_all(tasks).await;
}
