# CrystalAPI

## Concept:

the Crystal API will be a set of functions and objects that make it relatively easy for anyone to develop an application
for the crystal operating system.
this means that anyone with the API documentation and source code can easily make a program that follows the 

### current features:
as of 24/01/23 the API allows for the following:
- standard input (String)
- standard output with regular or coloured text
- detecting individual keystrokes
- access to some basic system information

### short term planned features:
- a greater set of OS information that the program can access
	- this will also include a module that the OS and other programs can read from to change the active state of the OS
	- this means that different applications can communicate with one another to share information.
- better support for coloured / formatted text
	- applications should be able to control when the user can enter keystrokes
		- this will have to be implemented in shell.rs with the text input so that the user cannot backspace text
		that has been written by the system or the application.
- text sandbox mode
	- this would essentially give the program more direct access to the vga buffer through some kind of wrapper function 
	/ class that would grant the ability to make a much more flexible interface.
	- the main benefit of this would be the ability for a developer to make simple 2d games by using characters on the vga
	buffer as pixels
