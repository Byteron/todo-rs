mod command;
mod todo;

use command::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    let mut list = todo::load();

    let command = command::parse(args);

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
        Command::Exit => {}
    }

    todo::save(list);
}
