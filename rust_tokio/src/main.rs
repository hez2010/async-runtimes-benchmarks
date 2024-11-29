use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let num_tasks = args[1].parse::<i32>().unwrap();
    let tasks = (0..num_tasks)
        .map(|_| sleep(Duration::from_secs(10)))
        .collect::<Vec<_>>();

    for task in tasks {
        task.await;
    }
}
