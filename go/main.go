package main

// Use context for potential timeout
// Add error tracking
// Implement a more structured goroutine approach

// More improvements:
// Add runtime memory profiling
// Implement more sophisticated context management
// Add tracing capabilities

import (
    "context"
    "fmt"
    "log"
    "os"
    "runtime"
    "runtime/trace"
    "strconv"
    "sync"
    "time"
)

func getMemoryStats() uint64 {
    var m runtime.MemStats
    runtime.ReadMemStats(&m)
    return m.Alloc
}
type TaskResult struct {
    TaskID       int
    StartMemory  uint64
    EndMemory    uint64
    StartTime    time.Time
    EndTime      time.Time
    Duration     time.Duration
}


func performTask(ctx context.Context, taskID int, results chan<- TaskResult, errChan chan<- error, wg *sync.WaitGroup) {
    defer wg.Done()

    startMemory := getMemoryStats()
    startTime := time.Now()

    select {
    case <-time.After(10 * time.Second):
        endTime := time.Now()
        endMemory := getMemoryStats()

        results <- TaskResult{
            TaskID:      taskID,
            StartMemory: startMemory,
            EndMemory:   endMemory,
            StartTime:   startTime,
            EndTime:     endTime,
            Duration:    endTime.Sub(startTime),
        }
    case <-ctx.Done():
        errChan <- ctx.Err()
    }
}

func main() {
	numRoutines := 100000
	if len(os.Args) > 1 {
		n, err := strconv.Atoi(os.Args[1])
		if err == nil {
			numRoutines = n
		}
	}

	ctx, cancel := context.WithTimeout(context.Background(), 11*time.Second)
	defer cancel()

	var wg sync.WaitGroup
	results := make(chan TaskResult, numRoutines)
	errChan := make(chan error, numRoutines)

	for i := 0; i < numRoutines; i++ {
		wg.Add(1)
		go performTask(ctx, i, results, errChan, &wg)
	}

	// Wait for all goroutines to complete
	go func() {
		wg.Wait()
		close(results)
		close(errChan)
	}()

	// Optional: Error handling and result processing
	for {
		select {
		case result, ok := <-results:
			if !ok {
				return
			}
			fmt.Printf("Task %d completed in %.2f seconds\n", result.TaskID, result.Duration.Seconds())
		case err := <-errChan:
			if err != nil {
				fmt.Println("Error:", err)
			}
		}
	}
}