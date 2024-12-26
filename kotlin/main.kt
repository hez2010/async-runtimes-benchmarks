import kotlinx.coroutines.*
import java.time.Instant
import java.time.Duration

data class TaskResult(
    val taskId: Int,
    val startTime: Instant,
    val endTime: Instant,
    val duration: Duration
)

suspend fun performTask(taskId: Int): TaskResult {
    val startTime = Instant.now()
    delay(10000) // 10-second delay
    val endTime = Instant.now()
    
    return TaskResult(
        taskId = taskId,
        startTime = startTime,
        endTime = endTime,
        duration = Duration.between(startTime, endTime)
    )
}

suspend fun main(args: Array<String>) {
    // Parse number of tasks, default to 100,000
    val numTasks = args.firstOrNull()?.toIntOrNull() ?: 100_000

    // Use coroutineScope for structured concurrency
    coroutineScope {
        val results = (0 until numTasks).map { taskId ->
            async {
                performTask(taskId)
            }
        }.awaitAll()

        // Optional: Print results
        results.forEach { result ->
            println("Task ${result.taskId} completed in ${result.duration.seconds} seconds")
        }
    }
}