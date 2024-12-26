import java.lang.management.ManagementFactory;
import java.lang.management.MemoryMXBean;
import java.lang.management.MemoryUsage;
import java.time.Instant;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

// Add JVM memory profiling
// Implement more detailed thread lifecycle tracking
// Use java.lang.management for comprehensive metrics

public class VirtualThreadBenchmarkImproved {
    static class TaskResult {
        int taskId;
        Instant startTime;
        Instant endTime;
        Duration duration;
        long memoryBefore;
        long memoryAfter;

        TaskResult(int taskId, Instant startTime, Instant endTime, long memoryBefore, long memoryAfter) {
            this.taskId = taskId;
            this.startTime = startTime;
            this.endTime = endTime;
            this.duration = Duration.between(startTime, endTime);
            this.memoryBefore = memoryBefore;
            this.memoryAfter = memoryAfter;
        }
    }

    static TaskResult performTask(int taskId) {
        MemoryMXBean memoryMXBean = ManagementFactory.getMemoryMXBean();
        MemoryUsage heapMemoryBefore = memoryMXBean.getHeapMemoryUsage();
        
        Instant startTime = Instant.now();
        try {
            Thread.sleep(10000);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        Instant endTime = Instant.now();

        MemoryUsage heapMemoryAfter = memoryMXBean.getHeapMemoryUsage();

        return new TaskResult(
            taskId, 
            startTime, 
            endTime, 
            heapMemoryBefore.getUsed(), 
            heapMemoryAfter.getUsed()
        );
    }

    public static void main(String[] args) {
        // Parse number of tasks, default to 100,000
        int numTasks = args.length > 0 ? 
            Integer.parseInt(args[0]) : 100_000;

        // Use Virtual Threads (Java 19+)
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            List<TaskResult> results = new ArrayList<>();

            // Spawn tasks
            for (int i = 0; i < numTasks; i++) {
                final int taskId = i;
                executor.submit(() -> {
                    TaskResult result = performTask(taskId);
                    synchronized (results) {
                        results.add(result);
                    }
                });
            }

            // Shutdown and wait for completion
            executor.shutdown();
            executor.awaitTermination(15, TimeUnit.SECONDS);

            // Optional: Print results
            results.forEach(result -> 
                System.out.printf("Task %d completed in %.2f seconds%n", 
                    result.taskId, 
                    result.duration.toSeconds())
            );
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }
}