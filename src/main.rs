mod todo;

use std::env;
use todo::Command;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return }

	let mut list = todo::load();

	let command = match args[1].as_str() {
		"list" => Command::List,
		"add" => Command::Add(args[2].clone()),
		"tick" => Command::Tick(args[2].clone().trim().parse().expect("Failed to parse Argument")),
		"untick" => Command::Untick(args[2].clone().trim().parse().expect("Failed to parse Argument")),
		"remove" => Command::Remove(args[2].clone().trim().parse().expect("Failed to parse Argument")),
		"reset" => Command::Reset,
		"exit" => Command::Exit,
		_ => Command::Unknown(args[1].clone()),
	};

	match command {
		Command::List => {
			list.print();
		}
		Command::Add(task) => {
			list.add_new(task.as_str());
		}
		Command::Tick(task_id) => {
			list.mark_completed(task_id, true);
		}
		Command::Untick(task_id) => {
			list.mark_completed(task_id, false);
		}
		Command::Remove(task_id) => {
			list.remove(task_id);
		}
		Command::Reset => {
			list.reset();
		}
		Command::Unknown(command) => {
			println!("Unknown Command {}", command);
		}
		Command::Exit => {},
	}

	todo::save(list);
}