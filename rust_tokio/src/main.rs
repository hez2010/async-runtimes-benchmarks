use std::env;
use tokio::{spawn, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let num_tasks = args[1].parse::<i32>().unwrap();
    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        tasks.push(spawn(sleep(Duration::from_secs(10))));
    }
    for task in tasks {
        task.await.unwrap();
    }
}