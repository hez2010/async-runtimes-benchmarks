use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    futures::future::join_all((0..num_tasks).map(|_| tokio::time::sleep(Duration::from_secs(10))))
        .await;
}
