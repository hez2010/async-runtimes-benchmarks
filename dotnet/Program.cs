using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

/// <summary>
/// Advanced .NET 9 Async Tasks Benchmark
/// 
/// Key Features:
/// - Leveraging .NET 9 performance improvements
/// - Native AOT compilation support
/// - Advanced memory profiling
/// - Detailed logging
/// - Configurable concurrency
/// </summary>
public class AsyncTasksBenchmark
{
    /// <summary>
    /// Detailed task performance metrics with .NET 9 enhancements
    /// </summary>
    public record TaskMetrics(
        int TaskId, 
        DateTime StartTime, 
        DateTime EndTime, 
        long StartMemory, 
        long EndMemory)
    {
        /// <summary>
        /// Calculate task duration using .NET 9 time abstractions
        /// </summary>
        public TimeSpan Duration => EndTime - StartTime;

        /// <summary>
        /// Calculate memory change with improved precision
        /// </summary>
        public long MemoryChange => EndMemory - StartMemory;
    }

    /// <summary>
    /// Advanced memory profiler optimized for .NET 9
    /// </summary>
    private static class MemoryProfiler
    {
        /// <summary>
        /// Get current process memory usage with .NET 9 improvements
        /// </summary>
        public static long GetMemoryUsage()
        {
            // Leverage .NET 9 cross-platform memory tracking
            return GC.GetTotalMemory(false);
        }
    }

    /// <summary>
    /// Perform an asynchronous task with comprehensive tracking
    /// </summary>
    private static async Task<TaskMetrics> PerformTaskAsync(
        int taskId, 
        ILogger logger)
    {
        // Leverage .NET 9's improved task tracking
        var startTime = DateTime.UtcNow;
        var startMemory = MemoryProfiler.GetMemoryUsage();

        try
        {
            // Simulate 10-second task with improved delay mechanism
            await Task.Delay(TimeSpan.FromSeconds(10));

            var endTime = DateTime.UtcNow;
            var endMemory = MemoryProfiler.GetMemoryUsage();

            // Advanced logging with structured logging
            logger.LogInformation(
                "Task {TaskId} completed. Duration: {Duration}ms, Memory Change: {MemoryChange} bytes", 
                taskId, 
                (endTime - startTime).TotalMilliseconds,
                endMemory - startMemory
            );

            return new TaskMetrics(
                TaskId: taskId, 
                StartTime: startTime, 
                EndTime: endTime, 
                StartMemory: startMemory, 
                EndMemory: endMemory
            );
        }
        catch (Exception ex)
        {
            // Enhanced error handling with .NET 9 logging
            logger.LogError(
                ex, 
                "Task {TaskId} failed with error: {ErrorMessage}", 
                taskId, 
                ex.Message
            );

            throw;
        }
    }

    /// <summary>
    /// Run benchmark with configurable task count
    /// </summary>
    private static async Task RunBenchmarkAsync(
        int numTasks, 
        ILogger logger)
    {
        // Leverage .NET 9's improved concurrent collections
        var results = new ConcurrentBag<TaskMetrics>();

        // Parallel task execution with improved task management
        var tasks = Enumerable.Range(0, numTasks)
            .Select(taskId => Task.Run(async () => 
            {
                var result = await PerformTaskAsync(taskId, logger);
                results.Add(result);
                return result;
            }))
            .ToList();

        // Efficient task completion tracking
        await Task.WhenAll(tasks);

        // Comprehensive benchmark summary
        logger.LogInformation(
            "Benchmark completed. Total Tasks: {TotalTasks}, Successful Tasks: {SuccessfulTasks}", 
            numTasks, 
            results.Count
        );
    }

    /// <summary>
    /// Main entry point with Native AOT support
    /// </summary>
    public static async Task Main(string[] args)
    {
        // Improved logging configuration
        using var loggerFactory = LoggerFactory.Create(builder => 
        {
            builder
                .AddConsole()
                .SetMinimumLevel(LogLevel.Information);
        });
        var logger = loggerFactory.CreateLogger<AsyncTasksBenchmark>();

        // Parse task count with improved parsing
        int numTasks = args.Length > 0 
            ? int.Parse(args[0]) 
            : 100_000;

        // Benchmark execution
        await RunBenchmarkAsync(numTasks, logger);
    }
}