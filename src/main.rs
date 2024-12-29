use todo::core::tasks::{Task, TaskList};

fn main() {
    let comer= Task::new_task(Some("comer"));
    let thao1 = Task::new_task(Some("Decirle a thao lo mucho que la quiero <3"));
    let thao2 = Task::new_task(Some("Darle muchos besitos  a thao :("));
    let thao3 = Task::new_task(Some("Cocianr algo rico para mi noviecita "));

    let tasks : TaskList = TaskList{items: vec![comer, thao1, thao2, thao3]};

    tasks.get_info();

}
