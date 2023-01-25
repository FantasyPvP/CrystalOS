use alloc::{string::String, vec::Vec, boxed::Box};
use crate::applications::shell::{
	Application,
	Error
};
use crate::{print, println};
use lazy_static::lazy_static;
use spin::Mutex;
use async_trait::async_trait;
use alloc::{
	string::ToString,
	borrow::ToOwned,
};

lazy_static! {
	static ref TASKS: Mutex<TaskList> = Mutex::new(TaskList::new());
}




pub struct Tasks;

#[async_trait]
impl Application for Tasks {
	fn new() -> Self { Self {} }

	async fn input(&mut self) -> String {
		String::from("e")
	}
	async fn keystroke(&mut self) -> char {
		'e'
	}
	async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {

		if args[0].clone() == String::from("add") {

			let content = args[2..].to_owned().into_iter().map(|mut s| {s.push_str(" "); s} ).collect::<String>();
			println!("added {}:\n    {}", &args[1], content);
			self.add_task(args[1].clone(), content);

		}
		if args[0].clone() == String::from("list") {
			
			for task in TASKS.lock().tasks.iter() {

				let idx = task.taskid;
				let taskname = task.name.clone();
				let content = task.content.clone();
				println!("    | Task {}  -> {}\n    | {}", idx, taskname, content);
			}

			
		}
		
		Ok(())
	}
}

impl Tasks {
	fn add_task(&mut self, name: String, content: String) {
		TASKS.lock().add(name, content);
	}
	fn remove_task(&self, index: usize) {
		
	}
	fn display(&self) {
		
	}
}







pub struct TaskList {
	current: usize,
	tasks: Vec<Task>,
	next_idx: usize,
}

impl TaskList {
	pub fn new() -> Self {
		Self {
			current: 0,
			tasks: Vec::new(),
			next_idx: 1
		}
	}
	pub fn next(&self) -> usize {
		self.next_idx
	}
	pub fn add(&mut self, name: String, content: String) {
		let task = Task::new(self.next(), name, content);
		let id = task.taskid.clone();
		self.tasks.push(task);
		self.current = id;
	}
	pub fn remove(&mut self, id: usize) {
		
	}
}




pub struct Task {
	taskid: usize,
	name: String,
	content: String,
}

impl Task {
	fn new(id: usize, name: String, content: String) -> Self {
		Self {
			taskid: id,
			name,
			content,
		}
	}
}
