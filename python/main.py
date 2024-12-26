import asyncio
import sys
import time
import logging
import memory_profiler

"""
Add explicit task creation with asyncio.create_task()
Implement a proper task function
Add error handling
Use asyncio.gather() with return_exceptions=True

Add memory profiling using memory_profiler
Implement logging with timestamps
Add command-line argument for memory tracking
Create a requirements.txt for dependencies
"""
# Configure logging
logging.basicConfig(
    level=logging.INFO, 
    format='%(asctime)s - %(levelname)s: %(message)s'
)

@memory_profiler.profile
async def perform_task(task_id):
    """
    Simulate a task with memory and performance tracking
    """
    start_memory = memory_profiler.memory_usage()[0]
    start_time = time.time()
    
    await asyncio.sleep(10)
    
    end_time = time.time()
    end_memory = memory_profiler.memory_usage()[0]
    
    logging.info(f"Task {task_id}: "
                 f"Duration={end_time-start_time:.2f}s, "
                 f"Memory Change={end_memory-start_memory:.2f}MB")
    
    return {
        'task_id': task_id,
        'duration': end_time - start_time,
        'memory_start': start_memory,
        'memory_end': end_memory
    }

async def main(num_tasks):
    """
    Create and run specified number of concurrent tasks.
    
    Args:
        num_tasks (int): Number of concurrent tasks to create
    """
    tasks = []
    for task_id in range(num_tasks):
        task = asyncio.create_task(perform_task(task_id))
        tasks.append(task)
    
    # Wait for all tasks to complete, capturing any exceptions
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # Optional: Basic error handling and logging
    for result in results:
        if isinstance(result, Exception):
            print(f"Task failed: {result}")

if __name__ == "__main__":
    # Parse number of tasks from command line, with a default
    num_tasks = int(sys.argv[1]) if len(sys.argv) > 1 else 100000
    
    # Run the async main function
    asyncio.run(main(num_tasks))

