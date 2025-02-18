# CrystalOS

the initial aim of this project was to follow a blog series on how to write an operating system in Rust (see links below)

https://os.phil-opp.com/
https://github.com/phil-opp/blog_os

After reading and implementing the features from the final chapter, (async/await) I could find no further instruction on how to continue 
from that point onwards. The author of the project is working on a third edition but there's no news of when it will be complete.

while I'm waiting for the third edition to release, I guess I'm gonna just have to improvise :)

- for more details on this project read the wiki ^^^

# Building
- this project unfortunately does not build on the latest rust versions and requires an older nightly build. 
the version below (rust v1.68.0-nightly) works:
> rustup override set nightly-2023-01-01
  (you may need to install this first)
> rustup install nightly-2023-01-01
- you will need the bare metal x86 target installed:
> rustup target add x86_64-unknown-none
- you will also need several extra rustup components to build the project
> rustup component add llvm-tools-preview
> rustup component add rust-src
  (ensure that you are installing the components for the correct nightly build)
- you will also need the bootimage crate which can be installed with the below command:
> cargo install bootimage
- the final requirement is having QEMU desktop installed for your system. on linux this comes in the default repositories of most major distributions.
- finally:
> cargo run


# Features as of Nov 2023

### barebones standard library with the following general features
  - random library for random choice and random integers
  - simple timing library to add delay
    - the delay is not at all accurate as it was hastily put together, this definitely needs to be rewritten
      when i have some more free time XD
  - Stdin and Stdout structs with all the following actions
    - individual keystroke input
    - string input (like a standard terminal)
    - print and println macros for output,
      - same macros are implemented for logging and for serial output (untested using real hardware, does definitely work in QEMU)
  - renderer for sending 'frames' to an 80x25 character ascii display (VGA Mode), these can be arrays of characters, or arrays
    of coloured characters, allowing for more advanced formatting if you need an application to support it.
    - additionally, a trait called element is provided, this will likely be changed significantly in future versions of the OS so the
      details on it's functionality are not important
  - application trait that provides a standardised way for applications to run in the OS.

### some basic apps
  - a To-Do list app
    - you can type 'tasks add <task name>' to add a task
    - you can also remove tasks and list them with 'tasks list'
  - snake game
    - pretty obvious what this is but there's also an impossible / chaos mode which adds
      a significant number of AI snakes that aggressively try to pursue the same points of interest
      as the player which can make survival challenging
      (basically a janky snake game crossed with slither.io)
  - conway's game of life
    - just makes some cool patterns, hardcoded starting pattern at the moment but I kinda want to add an editor for it in the future
  - a calculator that uses the same concepts as a compiler / interpreter to evaluate expressions
    - works for arithmetic expressions including powers, also supports functions like ln() and sqrt()
      - adding more functions in the future, recently covered taylor series in A levels so i'm implementing trig functions.
  - it can also graph stuff ig, using "graph" and putting an expression in terms of x with substitute numbers in for x and graph
    y vs x

  - a shell that can enter apps and run commands like 'echo' and 'clear'
    - well actually just those commands lol. Might try making a shell language or something if i get some spare time over christmas
