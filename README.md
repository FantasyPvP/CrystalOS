# CrystalOS

## Phase 1: The kernel.

the initial aim of this project was to follow a blog series on how to make a custom operating system found here:

https://os.phil-opp.com/

with the github repo for his project here:

https://github.com/phil-opp/blog_os

After reading and implementing the features from the final chapter, (async/await) I could find
no further instruction on how to continue with the project from there despite the author of the
series saying over a year previously that there would be more posts coming soon. 

i guess im gonna just have to improvise :)

the blog got me through the memory management side of the process so i believe that I should
have a lot more breathing room to implement the features that i want. As of completing the
tutorial, i obviously still dont have access to a standard library, however i can at least
use Vectors and Strings now which are important types, as well as the fact that i have access
to async and heap allocation

### my aims going forwards:

- whenever i have the chance to work on this project, i want to try and implement a new utility
which could be useful or cool for anyone using the operating system.
  - this could be anything from a cool neofetch style ascii fetcher (if you dont know what im 
talking about, its just a cool ascii logo of the operating system that appears when you open
a terminal sometimes)
- improve the text rendering system to create a set of globally accessible functions and/or macros
in order to render the text in a more visually appealing way to the user (as the default yellow text
does look extremely ugly lmao)
- implement a basic text editor (this will be difficult)
  - i would need a way to move the cursor around the screen and print text at that location
  - this would mean rewriting the majority of the code for the vga buffer module to create a more
  flexible system which allows for applications (modules / commands) to take more direct control of
  the text rendering whenever they are active


# Implementation

## Phase 2: the shell.

### shell.rs

diverging from the original blog series, i have made some significant changes to keyboard.rs
- i have moved the source code that handles the keyboard input from keyboard.rs to shell.rs
- this means that instead of the operating system running a task on startup that continually
awaits a the next keystroke and works from there, the new layout works very differently
- firstly, i use a lazy_static creating a static called CMD which houses the shell itself
- this allows me to reference it from anywhere in the code and initialise it as soon as the program
runs
- this may be changed later as i could just make an init function in shell.rs if i needed to
- the shell contains a get_input function that awaits a keystroke from the user before continuing
- this is looped inside the main shell function and added to a buffer
- when the \n character is inputted, the buffer is copied to the command history vector and then cleared
- additionally the buffer is run through a match statement that will start any app that matches the command
or alias.

## Phase 3: CrystalAPI

### the basics:

the crystal api will essentially be a standard library for any programs that are run by the shell
- it provides basic functions such as waiting for a keystroke or string to be entered by the user
- it will eventually support coloured text output once ive had a chance to modify the code for the vga
buffer to support coloured text output through a public function.

### example:
here is a template that could be used to program using the crystal API

```rust

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
```

## future plans:

eventually i want to try rewriting the majority of the code for the VGA buffer.
this is so that i can implement what i'll call a 'sandbox mode' for the screen.
this mode will support:
- moving the cursor around with arrow keys
- writing text at the cursor
- writing coloured text anywhere
- reading the entire output of the vga buffer or just a line into a string
eventually, this could theoretically lead to a library that was able to support things like a basic text editor 
for writing out messages and the capability to theoretically program basic 2d games in an ascii art style
(something like space invaders, tetris, etc.)

