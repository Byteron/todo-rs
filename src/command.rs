pub enum Command {
    List,
    Add(String),
    Remove(i32),
    Tick(i32),
    Untick(i32),
    Reset,
    Exit,
    Unknown(String),
}

pub fn parse(args: Vec<String>) -> Command {
    match args[1].as_str() {
        "list" => Command::List,
        "add" => Command::Add(args[2].clone()),
        "tick" => Command::Tick(
            args[2]
                .clone()
                .trim()
                .parse()
                .expect("Failed to parse Argument"),
        ),
        "untick" => Command::Untick(
            args[2]
                .clone()
                .trim()
                .parse()
                .expect("Failed to parse Argument"),
        ),
        "remove" => Command::Remove(
            args[2]
                .clone()
                .trim()
                .parse()
                .expect("Failed to parse Argument"),
        ),
        "reset" => Command::Reset,
        "exit" => Command::Exit,
        _ => Command::Unknown(args[1].clone()),
    }
}
