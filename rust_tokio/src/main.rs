use std::env;
use tokio::{spawn, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        tasks.push(spawn(sleep(Duration::from_secs(10))));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
