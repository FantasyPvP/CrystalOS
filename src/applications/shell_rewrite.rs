
/*

 [ Cry-SH ]

CrystalOS shell rewrite to replace the original shell implementation
this shell should support:
 - running basic commands
 - a prompt that displays the status of the last command
 - customised error messages returned from applications
 - invoking any application with arguments
 - cycling through previous commands with arrow keys
 - parsing of basic mathematical expressions using the calc module
 - chained commands using the '|' or pipe operator which sends the output
   of one command to the next
*/

// import necessary modules

use async_trait::async_trait;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use alloc::{string::{String, ToString}, vec::Vec, boxed::Box};

use crate::{
	kernel::tasks::keyboard::KEYBOARD,
	std::application::{Error, Application}
	std::io::{println, print};
};

use super::*
