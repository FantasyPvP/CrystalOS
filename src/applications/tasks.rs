use alloc::{string::String, vec::Vec, boxed::Box};
use crate::shell::{
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

			let content = args[1..].to_owned().into_iter().map(|mut s| {s.push_str(" "); s} ).collect::<String>();
			self.add_task(content);

		}

		if args[0].clone() == String::from("remove") {
			let idx = match  args[1].to_owned().parse::<usize>() {
				Ok(x) => x,
				Err(_) => { return Err(Error::CommandFailed(String::from("number must be an integer"))) },
			};
			self.remove_task(idx);
		}
		
		if args[0].as_str() == "list" {

			println!(
"-------------------------------------
         Your TODO List:
-------------------------------------\n");
			
			for task in TASKS.lock().tasks.iter() {

				let idx = task.taskid;
				let content = task.content.clone();
				println!("    | Task -> {} \n    | {}\n", idx, content);
			}
println!("\n-------------------------------------");
			
		}
		
		Ok(())
	}
}

impl Tasks {
	fn add_task(&mut self, content: String) {
		TASKS.lock().add(content);
	}
	fn remove_task(&self, idx: usize) {
		TASKS.lock().remove(idx);
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
	pub fn next(&mut self) -> usize {
		self.next_idx += 1;
		self.next_idx -1
	}
	pub fn add(&mut self, content: String) ->  Result<(), Error> {
		let task = Task::new(self.next(), content);
		let id = task.taskid.clone();
		self.tasks.push(task);
		self.current = id;
		Ok(())
	}
	pub fn remove(&mut self, id: usize) -> Result<(), Error> {
		for (i, task) in self.tasks.clone().iter().enumerate() {
			match task.taskid {
				id => { self.tasks.remove(i); },
				_ => { return Err(Error::CommandFailed(String::from("this task does not exist"))); },
			}
		};
		Ok(())
	}
}



#[derive(Debug, Clone)]
pub struct Task {
	taskid: usize,
	content: String,
}

impl Task {
	fn new(id: usize, content: String) -> Self {
		Self {
			taskid: id,
			content,
		}
	}
}
