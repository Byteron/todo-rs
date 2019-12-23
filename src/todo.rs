
#[derive(Debug)]
pub struct TodoItem {
	pub id: i32,
	pub description: String,
	pub completed: bool,
}

impl TodoItem {
	pub fn new(id: i32, description: &str) -> Self {
		TodoItem { id, description: description.to_string(), completed: false }
	}

	pub fn print(&self) {
		println!(
			"#{} [{}] - {}", 
			self.id,
			if self.completed { "x" } else { " " }, 
			self.description
		);
	}
}

pub struct TodoList {
	pub counter: i32,
	pub list: Vec<TodoItem>,
}

impl TodoList {
	pub fn new() -> Self {
		TodoList { counter: 0i32, list: Vec::new() }
	}

	pub fn add(&mut self, item: TodoItem) {
		self.list.push(item);
		self.counter += 1;
	}

	pub fn add_new(&mut self, description: &str) {
		let item = TodoItem::new(self.counter, description);
		self.add(item);
	}

	pub fn mark_done(&mut self, id: i32) {
		self.list.iter_mut()
			.filter(|item| item.id == id)
			.for_each(|item| item.completed = true);
	}

	pub fn print(&self) {
		for item in &self.list {
			item.print();
		}
	}
}