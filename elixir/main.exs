defmodule AsyncBenchmark do
  defstruct [:task_id, :start_time, :end_time, :duration]

  def perform_task(task_id) do
    start_time = :os.system_time(:millisecond)
    
    # Simulate 10-second task
    :timer.sleep(10000)
    
    end_time = :os.system_time(:millisecond)
    duration = end_time - start_time

    %__MODULE__{
      task_id: task_id,
      start_time: start_time,
      end_time: end_time,
      duration: duration
    }
  end

  def spawn_tasks(num_tasks) do
    parent = self()
    
    tasks = Enum.map(0..(num_tasks - 1), fn task_id ->
      spawn_link(fn ->
        result = perform_task(task_id)
        send(parent, {:task_result, result})
      end)
    end)

    # Collect results
    results = Enum.map(0..(num_tasks - 1), fn _ ->
      receive do
        {:task_result, result} -> result
      end
    end)

    # Optional: Print results
    Enum.each(results, fn result ->
      IO.puts "Task #{result.task_id} completed in #{result.duration / 1000} seconds"
    end)
  end

  def main(args \\ []) do
    # Parse number of tasks, default to 100,000
    num_tasks = case args do
      [num_str] -> String.to_integer(num_str)
      _ -> 100_000
    end

    spawn_tasks(num_tasks)
  end
end

# Run the benchmark
AsyncBenchmark.main(System.argv())