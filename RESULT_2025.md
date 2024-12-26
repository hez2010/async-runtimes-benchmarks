# Async Runtimes Benchmarks 2025

## Benchmark Evolution and Methodology

### Purpose

This benchmark aims to measure the memory footprint and performance characteristics of concurrent tasks across different programming language runtimes in 2025.

### Benchmark Specification

- Spawn N concurrent tasks (controlled by command-line argument)
- Default task count: 100,000
- Each task waits for 10 seconds
- Program exits when all tasks complete

## Improvements in 2025 Benchmark

### Standardization Efforts

1. **Consistent Task Implementation**
   - Explicit task creation mechanism - 2024 previous implementations vary slightly in how they create and manage tasks. We should standardize.
   - Standardized 10-second wait
   - Detailed task result tracking
   - Comprehensive error handling
   - Detailed performance tracking

2. **Language-Specific Enhancements**
   - Python (asyncio):
     - Explicit task creation with `asyncio.create_task()`
     - Comprehensive error handling
     - Detailed task result tracking

   - Node.js:
     - Promise-based task creation
     - Error handling with `Promise.allSettled()`
     - Detailed task duration tracking

   - Go:
     - Context-based timeout management
     - Structured goroutine approach
     - Error tracking and reporting

   - Rust (Tokio):
     - Detailed task tracking
     - Cancellation support
     - Comprehensive error handling

   - Kotlin (Coroutines):
     - TaskResult data class for comprehensive result tracking
     - Kotlin coroutines for concurrent task management
     - Detailed timing with Instant and Duration
     - coroutineScope for structured concurrency
     - awaitAll() for comprehensive task completion
     - Flexible task count with default of 100,000
     - Detailed result printing with task ID and duration

   - C# (.NET Tasks):
     - Introduced a record TaskResult for structured result tracking
     - Used Task.Run() for concurrent task creation
     - Implemented detailed timing with DateTime and TimeSpan
     - Used ConcurrentBag for thread-safe result collection
     - Comprehensive error handling
     - Flexible task count with default of 100,000
     - Optional result printing with task details

   - Elixir (Processes):
     - Created a custom struct for task result tracking
     - Used Elixir's lightweight processes for concurrency
     - Implemented detailed timing with system time
     - Used spawn_link for process creation
     - Message passing for result collection
     - Flexible task count with default of 100,000
     - Optional result printing with task ID and duration

### Benchmarking Recommendations

## Overarching Recommendations:

- Standardize memory tracking across languages
- Add detailed logging and tracing
- Implement consistent performance metrics
- Create a unified result collection and reporting mechanism
- Add configuration for different benchmark scenarios

1. **Consistent Environment**
   - Use identical hardware
   - Standardized OS and runtime versions
   - Minimal background processes

2. **Metrics Collection**
   - Memory usage
   - CPU utilization
   - Task creation overhead
   - Total execution time
   - Garbage collection impact

### Methodology Notes

- Multiple iterations for statistical significance
- Warm-up runs to stabilize runtime environments
- Detailed logging of task performance

### Future Improvements

- Implement similar benchmarking approach
- Add more language runtimes
- Implement more sophisticated memory profiling
- Create visualization tools for result comparison

## Conclusion

This benchmark provides insights into the concurrent task handling capabilities of different language runtimes, highlighting the evolving landscape of asynchronous programming in 2025.
