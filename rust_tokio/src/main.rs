use std::env;
use std::time::{Duration, Instant};
use tokio::{spawn, time::sleep, select};

// Add more detailed task tracking
// Implement proper error handling
// Use tokio::select! for potential cancellation

#[derive(Debug)]
struct TaskResult {
    task_id: usize,
    start_time: Instant,
    end_time: Instant,
    duration: Duration,
}

async fn perform_task(task_id: usize) -> TaskResult {
    let start_time = Instant::now();
    
    // Simulate 10-second task with potential for cancellation
    let sleep_future = sleep(Duration::from_secs(10));
    
    select! {
        _ = sleep_future => {
            let end_time = Instant::now();
            TaskResult {
                task_id,
                start_time,
                end_time,
                duration: end_time.duration_since(start_time),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Parse number of tasks from command line, default to 100,000
    let num_tasks: usize = env::args()
        .skip(1)
        .next()
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(100_000);

    let mut tasks = Vec::with_capacity(num_tasks);
    
    // Spawn tasks
    for task_id in 0..num_tasks {
        tasks.push(spawn(perform_task(task_id)));
    }
    
    // Collect and process results
    for task in tasks {
        match task.await {
            Ok(result) => {
                println!(
                    "Task {} completed in {:.2} seconds", 
                    result.task_id, 
                    result.duration.as_secs_f64()
                );
            }
            Err(e) => {
                eprintln!("Task failed: {}", e);
            }
        }
    }
}