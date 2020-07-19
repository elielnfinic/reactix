use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {
    println!("Starting reactix ...");
    let args : Vec<String> = env::args().collect();

    for c in args.iter(){
        //println!("The arguments {} ",c);
    }

    //Run Command
    match Command::new("cargo").arg("new").arg("monsite").output(){
        Ok(_) => println!("Rust project created ..."),
        Err(_) => println!("Error creating rust project, check if rust is installed")
    };
    //Edition of the TOML file

    let mut write_file = OpenOptions::new().write(true).append(true).open("monsite/Cargo.toml").expect("Can't write to the file");
    if let Err(e) = writeln!(write_file,"actix=\"2.0\""){
        eprintln!("Could not write to file because {}",e);
    }

    //Create react project in the new project
    match Command::new("create-react-app").arg("monsite/frontend").output(){
        Ok(_) => println!("React project generated successfully..."),
        Err(_) => println!("Error creating React project")
    };
    
    //Start edition of the main file to add actix code

}
