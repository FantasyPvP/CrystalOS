use alloc::{string::String, vec::Vec, boxed::Box};
use crate::std::application::{
	Application,
	Error
};
use crate::println;
use crate::std::render::Window;
use lazy_static::lazy_static;
use spin::Mutex;
use async_trait::async_trait;
use alloc::{
	borrow::ToOwned,
};

use crate::std::random;


lazy_static! {
	static ref TASKS: Mutex<TaskList> = Mutex::new(TaskList::new());
}




pub struct Tasks;

#[async_trait]
impl Application for Tasks {
	fn new() -> Self { Self {} }

	async fn run(&mut self, window: Option<Window>, args: Vec<String>) -> Result<(), Error> {

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

		if args[0].clone() == String::from("select") {
			let arg2 = args[1].clone();
			if arg2 == String::from("random") {
				let len = TASKS.lock().tasks.len();
				self.select_task(random::Random::int(0, len -1) as i32);
			} else if arg2.parse::<u64>().is_ok() {
				()
			}
		}

		if args[0].clone() == String::from("priority") {
			let idx = TASKS.lock().current;
			if idx < 0 {
				println!(
"-------------------------------------
no task currently set as priority
-------------------------------------\n"
				);
				return Ok(())
			}

			let task = TASKS.lock().tasks[idx as usize].clone();
			let content = task.content.clone();

			println!(
"-------------------------------------
PRIORITY TASK: {} : {}
-------------------------------------\n",
				idx, content
			)
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
		TASKS.lock().add(content).unwrap();
	}
	fn remove_task(&self, idx: usize) {
		TASKS.lock().remove(idx).unwrap();
	}
	fn select_task(&self, idx: i32) {
		TASKS.lock().select(idx).unwrap();
	}
}

pub struct TaskList {
	current: i32,
	tasks: Vec<Task>,
	next_idx: usize,
}

impl TaskList {
	pub fn new() -> Self {
		Self {
			current: -1,
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
		let _id = task.taskid.clone();
		self.tasks.push(task);
		Ok(())
	}
	pub fn remove(&mut self, _id: usize) -> Result<(), Error> {
		for (i, task) in self.tasks.clone().iter().enumerate() {
			match task.taskid {
				_id => { self.tasks.remove(i); },
				_ => { return Err(Error::CommandFailed(String::from("this task does not exist"))); },
			}
		};
		Ok(())
	}
	pub fn select(&mut self, idx: i32) -> Result<(), Error> {
		self.current = idx;
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
