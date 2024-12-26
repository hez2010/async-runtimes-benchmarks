import asyncio
import sys
import time

"""
Add explicit task creation with asyncio.create_task()
Implement a proper task function
Add error handling
Use asyncio.gather() with return_exceptions=True
"""
async def perform_task(task_id):
    """
    Simulate a task that waits for 10 seconds.
    
    Args:
        task_id (int): Unique identifier for the task
    """
    start_time = time.time()
    await asyncio.sleep(10)
    end_time = time.time()
    return {
        'task_id': task_id,
        'duration': end_time - start_time
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

