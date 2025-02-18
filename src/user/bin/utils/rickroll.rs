use async_trait::async_trait;

use crate::std::application::{
	Application,
	Error,
};

const RICKROLL1: &str = "
  _   _                        _____
 | \\ | |                      / ____|
 |  \\| | _____   _____ _ __  | |  __  ___  _ __  _ __   __ _
 | . ` |/ _ \\ \\ / / _ \\ '__| | | |_ |/ _ \\| '_ \\| '_ \\ / _` |
 | |\\  |  __/\\ V /  __/ |    | |__| | (_) | | | | | | | (_| |
 |_| \\_|\\___| \\_/ \\___|_|     \\_____|\\___/|_| |_|_| |_|\\__,_|
   _______            __     __          ___  ___
  / ____(_)           \\ \\   / /          | |  | |
 | |  __ ___   _____   \\ \\_/ /__  _   _  | |  | |_ __
 | | |_ | \\ \\ / / _ \\   \\   / _ \\| | | | | |  | | '_ \\
 | |__| | |\\ V /  __/    | | (_) | |_| | | |__| | |_) |
  \\_____|_| \\_/ \\___|    |_|\\___/ \\__,_|  \\____/| .__/
                                                | |
                                                |_|";
const RICKROLL2: &str = "
            t0ObOmmQ            ,--.  ,--.
           {Cdbhkbpwq?          |  ,'.|  | ,---.,--.  ,--.,---. ,--.--.
           {wzXunrjjCJ          |  |' '  || .-. :\\  `'  /| .-. :|  .--'
           |mXYzrvrjnv          |  | `   |\\   --. \\    / \\   --.|  |
            OUCYLrcxjX          `--'  `--' `----'  `--'   `----'`--'
           LYUUJ0uvjr]           ,----.
            1CCCZvZx            '  .-./    ,---. ,--,--, ,--,--,  ,--,--.
             UJJOU1j            |  | .---.| .-. ||      \\|      \\' ,-.  |
            mXQQQUvC            '  '--'  |' '-' '|  ||  ||  ||  |\\ '-'  |
        |dooaj0COXcY(hhQ         `------'  `---' `--''--'`--''--' `--`--'
     tdaoaao*a)UvYz/}khhaaa	     ,----.   ,--.
   baoooahka**Y1}nX|[hhhahhk    '  .-./   `--',--.  ,--.,---.
   aoaoohkXhooo|)r1)ahkhaaoo    |  | .---.,--. \\  `'  /| .-. :
   **##*hLmnnmaJYpvnohhhaaook   '  '--'  ||  |  \\    / \\   --.
   b#**#00LXvoar11(v#hhhoo*oa    `------' `--'   `--'   `----'
   b*##MhL0zcaoaxwbhohaa****    ,--.   ,--.                 ,--. ,--.
  CQ***MJQQcoooaj|uzoooa*##*     \\  `.'  /,---. ,--.,--.    |  | |  | ,---.
 0a###w#*mObo*oatXkW*oo#*###p     '.    /| .-. ||  ||  |    |  | |  || .-. |
 mpZdMMM#p&M***adhM&##*##MMMk       |  | ' '-' ''  ''  '    '  '-'  '| '-' '
 1a###w#*mObo*oatXkW*oo#*###p       `--'  `---'  `----'      `-----' |  |-'
                                                                     `--'";

use crate::println;
use crate::std::render::Window;
use alloc::{string::String, boxed::Box, vec::Vec};


pub struct Rickroll {}

#[async_trait]
impl Application for Rickroll {
	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {
		println!("{}", RICKROLL2);
		Ok(())
	}
}

