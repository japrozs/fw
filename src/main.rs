use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
mod utils;

fn main(){
    let argv : Vec<String> = env::args().collect();
    
    if argv.len() == 1{
        // Print the help message
        utils::help_msg();
        process::exit(0);
    }else if argv[1] == "-h" || argv[1] == "--help"{
        // Print the help message
        utils::help_msg();
        process::exit(0);
    }

    let cmd = argv[1].clone();
    let mut files = vec![];
    let mut files_rel_path = vec![];
    for i in 2..argv.len(){
        let absolute_path = fs::canonicalize(&PathBuf::from(argv[i].clone()));
        let file = match absolute_path{
            Ok(msg) => msg,
            Err(_) => {
                println!("Error : \"{}\" : No such file", argv[i].clone());
                process::exit(0);
            }
        };
        files_rel_path.push(argv[i].clone());
        files.push(file);
    }

    /* Now create a file watcher for each of the files */
    utils::create_file_watcher(files, cmd, files_rel_path);
}