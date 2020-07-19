use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {
    println!("Starting reactix ...");

    let appname = "reactix-app";

    let args : Vec<String> = env::args().collect();

    //Reading the arguments
    for c in args.iter(){
        //println!("The arguments {} ",c);
    }

    //Run Command
    match Command::new("cargo").arg("new").arg(appname).output(){
        Ok(_) => println!("Rust project created ..."),
        Err(_) => println!("Error creating rust project, check if rust is installed")
    };
    //Edition of the TOML file

    let mut write_file = OpenOptions::new().write(true).append(true).open(appname.to_owned()+"/Cargo.toml").expect("Can't write to the file");
    if let Err(e) = writeln!(write_file,"actix=\"2.0\""){
        eprintln!("Could not write to file because {}",e);
    }

    //Create react project in the new project
    match Command::new("create-react-app").arg(appname.to_owned()+"/frontend").output(){
        Ok(_) => println!("React project generated successfully..."),
        Err(_) => println!("Error creating React project")
    };

    //Adding README.md to react project
    let mut write_to_readme = File::create(appname.to_owned() + "/frontend/README.md").expect("Unable to create front-end README.md file");
    write_to_readme.write_all(b"
        ## Front-end generated by [Reactix](https://github.com/elielnfinic/reactix) \n\n
        This is the ReactJS project that will run along with your Rust Actix web backend.\n\n
        ## Contribute to project here \n\n
        [https://github.com/elielnfinic/reactix](https://github.com/elielnfinic/reactix)\n\n
        ## Contact the developer\n\n
        I am available on Discord elielmathe#6699\n\n
        Follow me on Twitter [@elielmathe](https://twitter.com/elielmathe)\n\n
        Enjoy!\n\n
    ");
}

#[cfg(test)]
mod test{
    #[test]
    pub fn test_1(){
        assert_eq!(1,1);
    }
}
