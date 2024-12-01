use std::env;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let num_tasks = args[1].parse::<usize>().unwrap();
    futures::future::join_all((0..num_tasks).map(|_| tokio::time::sleep(Duration::from_secs(10))))
        .await;
}
