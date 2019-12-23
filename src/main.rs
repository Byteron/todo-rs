use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod todo;

use todo::TodoList;

fn main() {

	let mut list = load();

	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		return
	}

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

	save(list);
}

fn load() -> TodoList {
	let mut file = OpenOptions::new()
		.read(true).write(true).create(true)
		.open("todo.txt")
		.unwrap();
	
	let mut contents = String::new();	
	file.read_to_string(&mut contents).expect("Could not read file");

	let list = TodoList::from(contents);

	list
}

fn save(list: TodoList) {
	let mut file = OpenOptions::new()
	.read(true).write(true).create(true)
	.open("todo.txt")
	.unwrap();

	file.set_len(0).expect("Could not erase file");
	file.write_all(list.to_string().as_ref()).expect("Could not write to file.");
	file.sync_all().expect("Could not sync file.");
}