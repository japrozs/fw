extern crate notify;

/* External imports */
use notify::{Watcher, RecursiveMode, watcher};
use colored::*;

/* Std library imports */
use std::sync::mpsc::{channel,RecvError};
use std::time::Duration;
use std::process::Command;

pub fn help_msg(){
	println!(
"\n{}

USAGE :
\tfw \"CMD\" [FILES...]

OPTIONS:
-h | --help : Display this message

Created by {}
LICENSE : {}
Source code : {}\n",
	"fw : A filewatcher for all programming languages".bright_blue(),"MIT".green(), "Japroz Saini<sainijaproz@gmail.com>".green(), "https://github.com/japrozs/fw".green()
	)
}

pub fn err(error : RecvError){
	println!("Error : {}",error);
}

pub fn create_file_watcher(files : Vec<std::path::PathBuf>, cmd : String, files_rel_path : Vec<std::string::String>){
	let (tx, rx) = channel();
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    for file in files{
        watcher.watch(file, RecursiveMode::Recursive).unwrap();
    }

	let mut file_str = String::new();
		for file in &files_rel_path{
			file_str += &(format!("\"{}\" ", file));
		}
		println!("{}{}", "[fw] v".yellow(), env!("CARGO_PKG_VERSION").yellow());
		println!("{}", "[fw] to restart at any time, enter `rs`".yellow());
		println!("{}{}", "[fw] watching path(s): ".yellow(),file_str.yellow());
		println!("{}{}{}","[fw] starting `".green(), cmd.green(), "`".green());

    loop {
        match rx.recv() {
           Ok(msg) => {
			println!("{:?}", msg);
			let output = if cfg!(target_os = "windows") {
				Command::new("cmd")
						.args(&["/C", &cmd])
						.output()
						.expect(&(format!("Failed to execute shell command : {}", cmd)))
			} else {
				Command::new("sh")
						.arg("-c")
						.arg(&cmd)
						.output()
						.expect(&(format!("Failed to execute shell command : {}", cmd)))
			};
			
			println!("{}",String::from_utf8_lossy(&output.stdout).bright_blue());
		   },
           Err(e) => err(e),
        }
		let mut file_str = String::new();
		for file in &files_rel_path{
			file_str += &(format!("\"{}\" ", file));
		}
		println!("{}", "[fw] 0.0.1".yellow());
		println!("{}", "[fw] to restart at any time, enter `rs`".yellow());
		println!("{}{}", "[fw] watching path(s): ".yellow(),file_str.yellow());
		println!("{}{}{}","[fw] starting `".green(), cmd.green(), "`".green());
    }
}