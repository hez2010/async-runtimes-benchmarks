int numTasks = int.Parse(args[0]);

List<Task> tasks = new List<Task>(numTasks);

for (int i = 0; i < numTasks; i++)
{
    tasks.Add(Task.Delay(TimeSpan.FromSeconds(10)));
}

await Task.WhenAll(tasks);
