//! # Rust Async Tasks Benchmark
//!
//! A comprehensive benchmark for measuring concurrent task performance
//! using Tokio runtime with advanced memory and performance tracking.
//!
//! ## Key Features
//! - Detailed task metrics
//! - Advanced memory tracking
//! - Comprehensive error handling
//! - Performance logging

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, error, Level};
use serde::{Serialize, Deserialize};

/// Comprehensive task performance metrics
#[derive(Debug, Serialize, Deserialize)]
struct TaskMetrics {
    /// Unique task identifier
    task_id: usize,
    
    /// Task start timestamp
    start_time: Instant,
    
    /// Task end timestamp
    end_time: Instant,
    
    /// Total task duration
    duration: Duration,
    
    /// Memory allocated during task
    memory_allocated: usize,
}

/// Global memory allocation tracker
static GLOBAL_MEMORY_TRACKER: AtomicUsize = AtomicUsize::new(0);

/// Custom allocator for precise memory tracking
struct MemoryTrackingAllocator<A: std::alloc::GlobalAlloc> {
    allocator: A,
}

unsafe impl<A: std::alloc::GlobalAlloc> std::alloc::GlobalAlloc for MemoryTrackingAllocator<A> {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let ptr = self.allocator.alloc(layout);
        if !ptr.is_null() {
            GLOBAL_MEMORY_TRACKER.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        self.allocator.dealloc(ptr, layout);
        GLOBAL_MEMORY_TRACKER.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

/// Perform an asynchronous task with comprehensive tracking
///
/// # Arguments
///
/// * `task_id` - Unique identifier for the task
///
/// # Returns
///
/// Detailed task performance metrics
async fn perform_task(task_id: usize) -> TaskMetrics {
    // Record start time and initial memory state
    let start_time = Instant::now();
    let start_memory = GLOBAL_MEMORY_TRACKER.load(Ordering::Relaxed);

    // Simulate 10-second task
    sleep(Duration::from_secs(10)).await;

    // Record end time and final memory state
    let end_time = Instant::now();
    let end_memory = GLOBAL_MEMORY_TRACKER.load(Ordering::Relaxed);

    // Construct and return task metrics
    TaskMetrics {
        task_id,
        start_time,
        end_time,
        duration: end_time.duration_since(start_time),
        memory_allocated: end_memory.saturating_sub(start_memory),
    }
}

/// Main benchmark execution function
///
/// # Arguments
///
/// * `num_tasks` - Number of concurrent tasks to create
async fn run_benchmark(num_tasks: usize) {
    // Create vector to hold task handles
    let mut tasks = Vec::with_capacity(num_tasks);

    // Spawn concurrent tasks
    for task_id in 0..num_tasks {
        let task = tokio::spawn(async move {
            match perform_task(task_id).await {
                Ok(metrics) => {
                    info!(
                        task_id = metrics.task_id,
                        duration_ms = metrics.duration.as_millis(),
                        memory_allocated = metrics.memory_allocated,
                        "Task completed successfully"
                    );
                    metrics
                }
                Err(e) => {
                    error!(task_id, error = %e, "Task failed");
                    panic!("Task failed: {}", e);
                }
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
}

/// CLI entry point with configurable task count
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Parse task count from CLI or use default
    let num_tasks = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(100_000);

    // Execute benchmark
    run_benchmark(num_tasks).await;
}