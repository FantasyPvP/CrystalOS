
// ignore everything from this point up until the App struct

// --------------OS-INTERFACE-------------------------------------------------------------------------------------------------------

use std::io;
use std::io::Write; // ignore these, i have my own implementations that i will replace them with

struct CommandHandler {} // a struct used in my code (just ignore)

impl CommandHandler { // dont modify anything here
    fn new() -> Self {
        Self {}
    }

    fn input(&mut self) -> String { // this function will get replaced by the custom input function
        let mut string = String::new();
        io::stdin().read_line(&mut string).expect("error getting input");
        string
    }
}

fn main() { // the entry point to your code, it calls the code for the application
            // will be removed when integrated into the os and replaced by the shell command
    println!("");
    print!("enter arguments to run command with > ");
    io::stdout().flush();
    let mut args = String::new();
    io::stdin().read_line(&mut args).expect("failed to get input");
    let mut app = App::new(CommandHandler::new());
    app.run(args);
}




// --------------IMPLEMENTATION-----------------------------------------------------------------------------------------------------


struct App { // change name to whatever you want
    handler: CommandHandler,
    // any global variables for the application should be put here
    // in the form:   varname: VarType,
}

impl App { // name must be the same as the name of the struct
    fn new(handler: CommandHandler) -> Self {
        Self { // this should add any variables that are needed while the application is running
            handler: handler,
        //  status: String,     (example)
        }
    }

    fn input(&mut self) -> String { // this function gives command line input
        self.handler.input()
    }

    fn keystroke(&mut self) -> String {
    	self.handler.input()
    }

    fn run(&mut self, args: String) -> Result<(), String> { /*
        this represents your actual main function
        write all the code for your program starting here

        use println!() to print to the screen
        use self.input() to get input from terminal
        */

            println!("app running {}", args); // do stuff here

        // example of how you can use the input function

            println!("type something");
            println!("input: {}", self.input());

        // if you want to return an error, write: return Err("error message")
        // the error message tells the operating system what went wrong with the code or user input.
        // if you want to return ok, write: return Ok(())  (make sure to have the 2 sets of brackets)
        Ok(())
    }
}





