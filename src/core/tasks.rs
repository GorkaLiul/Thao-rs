#[derive(Debug)]
pub enum Task {
    Todo { description: String, status: bool },
}

impl Task {
    pub fn new_task(description: Option<&str>) -> Self {
        Task::Todo { description: description.unwrap_or("").to_string(),
            status: false,
        }
    }


    fn get_info(&self) -> String {
        match self {
            Task::Todo {
                description,
                status,
            } => format!("Description: {}, Status: {}", description, status),
        }
    }
}

pub struct TaskList{ pub items : Vec<Task>}

impl TaskList{
    pub fn get_info(&self){
        println!("List size: {}", &self.items.len());
        for token in &self.items{
            match token {
                Task::Todo {description, status}=> println!("Description: {}", description),
            }
        }
    }
}

