mod todo;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return }

	let mut list = todo::load();

	let command: String = args[1].clone();

	match command.as_str() {
		"list" => {
			list.print();
		}
		"add" => {
			let task: String = args[2].clone();
			list.add_new(task.as_str());
		}
		"tick" => {
			let task: String = args[2].clone();
			let task_id: i32 = task.trim().parse().unwrap();
			list.mark_completed(task_id, true);
		}
		"untick" => {
			let task: String = args[2].clone();
			let task_id: i32 = task.trim().parse().unwrap();
			list.mark_completed(task_id, false);
		}
		"delete" => {
			let task: String = args[2].clone();
			let task_id: i32 = task.trim().parse().unwrap();
			list.remove(task_id);
		}
		"reset" => {
			list.reset();
		}
		_ => { println!("Unknown Command: {}", command); }
	}

	todo::save(list);
}