import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public class VirtualThreadBenchmark {
    // Structured task result tracking
    static class TaskResult {
        int taskId;
        Instant startTime;
        Instant endTime;
        Duration duration;

        TaskResult(int taskId, Instant startTime, Instant endTime) {
            this.taskId = taskId;
            this.startTime = startTime;
            this.endTime = endTime;
            this.duration = Duration.between(startTime, endTime);
        }
    }

    // Perform task with detailed tracking
    static TaskResult performTask(int taskId) {
        Instant startTime = Instant.now();
        try {
            // Simulate 10-second task
            Thread.sleep(10000);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        Instant endTime = Instant.now();
        return new TaskResult(taskId, startTime, endTime);
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