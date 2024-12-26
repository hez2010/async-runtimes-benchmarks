use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::{spawn, time::sleep, select};

// Add more detailed task tracking
// Implement proper error handling
// Use tokio::select! for potential cancellation

// More improvements:
// Add more comprehensive memory tracking
// Implement custom allocator tracing
// Add detailed performance metrics

// Custom allocator for memory tracking
struct TracingAllocator<A: GlobalAlloc> {
    allocator: A,
    allocated: AtomicUsize,
}


#[derive(Debug)]
struct TaskResult {
    task_id: usize,
    start_time: Instant,
    end_time: Instant,
    duration: Duration,
    memory_allocated: usize,
}


async fn perform_task(task_id: usize, allocator: &TracingAllocator<std::alloc::System>) -> TaskResult {
    let start_time = Instant::now();
    let start_memory = allocator.allocated.load(Ordering::Relaxed);
    
    sleep(Duration::from_secs(10)).await;
    
    let end_time = Instant::now();
    let end_memory = allocator.allocated.load(Ordering::Relaxed);

    TaskResult {
        task_id,
        start_time,
        end_time,
        duration: end_time.duration_since(start_time),
        memory_allocated: end_memory - start_memory,
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