#[derive(Debug)]
struct Task {
    id: u8,
    title: String,
    is_done: bool,
}

impl Task {
    fn create_task(id: u8, title: String) -> Task {
        Task {
            id,
            title,
            is_done: false,
        }
    }
}

struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
        println!("✅ Task added successfully!");
    }

    fn mark_complete(&mut self, id: u8) {
        let mut found = false;
        for task in &mut self.tasks {
            if task.id == id {
                task.is_done = true;
                println!("✅ Task '{}' marked as complete!", task.title);
                found = true;
                break;
            }
        }
        if !found {
            println!("❌ Task with ID {} not found!", id);
        }
    }

    fn delete_task(&mut self, id: u8) {
        let len_before = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        if self.tasks.len() < len_before {
            println!("🗑️ Task with ID {} deleted successfully!", id);
        } else {
            println!("❌ Task with ID {} not found!", id);
        }
    }

    fn list_tasks(&self) {
        println!("\n📋 Current Tasks:");
        if self.tasks.is_empty() {
            println!("(No tasks yet!)");
        } else {
            for task in &self.tasks {
                println!(
                    "ID: {}, Title: {}, Done: {}",
                    task.id, task.title, task.is_done
                );
            }
        }
    }
}

fn main() {
    let mut my_tasks = TaskList::new();

    println!("Enter:");
    println!("1️⃣ Add a new task");
    println!("2️⃣ Mark a task as complete");
    println!("3️⃣ Delete a task");
    println!("4️⃣ Exit");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num: u8 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        match num {
            1 => {
                println!("Enter task ID:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str).unwrap();
                let id: u8 = id_str.trim().parse().expect("Invalid ID!");

                println!("Enter task title:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();

                let task = Task::create_task(id, title.trim().to_string());
                my_tasks.add_task(task);
            }

            2 => {
                println!("Enter task ID to mark complete:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str).unwrap();
                let id: u8 = id_str.trim().parse().expect("Invalid ID!");
                my_tasks.mark_complete(id);
            }

            3 => {
                println!("Enter task ID to delete:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str).unwrap();
                let id: u8 = id_str.trim().parse().expect("Invalid ID!");
                my_tasks.delete_task(id);
            }

            4 => {
                println!("👋 Exiting Task Manager. Have a productive day!");
                break;
            }

            _ => println!("❌ Invalid option! Please enter 1–4."),
        }

        my_tasks.list_tasks();
        println!("\nEnter your next choice (1-4):");
    }
}
