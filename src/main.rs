use std::env;

mod todo;

use todo::TodoList;

fn main() {
	let args: Vec<String> = env::args().collect();
	let command: String = args[1].clone();

	let mut list = TodoList::new();

	list.add_new("This is Task One");
	list.add_new("This is Task Two");

	match command.as_str() {
		"list" => {
			list.print();
		}
		"add" => {
			let task: String = args[2].clone();
			list.add_new(task.as_str());
			list.print();
		}
		"complete" => {
			let task: String = args[2].clone();
			let task_id: i32 = task.trim().parse().unwrap();
			list.mark_completed(task_id, true);
			list.print();
		}
		"incomplete" => {
			let task: String = args[2].clone();
			let task_id: i32 = task.trim().parse().unwrap();
			list.mark_completed(task_id, false);
			list.print();
		}
		_ => { println!("Unknown Command: {}", command); }
	}
}