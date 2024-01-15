struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: &str) -> Task {
        Task {
            id,
            description: description.to_string(),
            completed: false,
        }
    }
}

struct ToDoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    fn add_task(&mut self, description: &str) -> &Task {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
        self.tasks.last().unwrap()
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        let task_option = self.tasks.iter_mut().find(|task| task.id == id);

        if let Some(task) = task_option {
            task.completed = true;
            Some(task as &Task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    
}
