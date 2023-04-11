use std::process::Command;
use crate::color::Color;


const BANNAR: &str = r#"
 ███▄ ▄███▓ ▄▄▄       ██▀███   ▄████▄   ▒█████  
▓██▒▀█▀ ██▒▒████▄    ▓██ ▒ ██▒▒██▀ ▀█  ▒██▒  ██▒
▓██    ▓██░▒██  ▀█▄  ▓██ ░▄█ ▒▒▓█    ▄ ▒██░  ██▒
▒██    ▒██ ░██▄▄▄▄██ ▒██▀▀█▄  ▒▓▓▄ ▄██▒▒██   ██░
▒██▒   ░██▒ ▓█   ▓██▒░██▓ ▒██▒▒ ▓███▀ ░░ ████▓▒░
░ ▒░   ░  ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░░ ░▒ ▒  ░░ ▒░▒░▒░ 
░  ░      ░  ▒   ▒▒ ░  ░▒ ░ ▒░  ░  ▒     ░ ▒ ▒░ 
░      ░     ░   ▒     ░░   ░ ░        ░ ░ ░ ▒  
       ░         ░  ░   ░     ░ ░          ░ ░  
                              ░                 
"#;

pub fn show() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    };

    println!("{BANNAR}");
    println!("Wellcome to my korea-romanization CLI project.\n");
    println!("This project's source code can be found in this project's GitHub repo.\n--\n");

    println!("My name is Aung Lynn Khant.");
    println!("Github: https://github.com/alk-neq-me\n");

    println!("Type `help` to see usage.\nType `q` to quit.\n");
    
    println!("-- \n{}", Color::Green(String::from("Usage: ")));
    println!("    type korea context `hangul` input.\n");
    
    println!("{}", Color::Green(String::from("Example: ")));
    println!("    hangul: 안녕하세요");
}
