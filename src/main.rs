use std::{env, process};
use minigrep_proj::Config;


fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);

    });

    println!("Searching for: {}",config.query );
    println!("In the file: {}", config.file_path);

    if let Err(e)= minigrep_proj::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
   
    

}

