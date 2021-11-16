use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
fn main() {
    loop {
        print!("🌽$ ");
        stdout().flush();

        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();
        let mut sections = inp.trim().split_whitespace();
        let input_fin = sections.next().unwrap();
        let arguments = sections;

        match input_fin {
            "cd" => {
                let new_dir = arguments.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,

            input_fin => {
                let mut new_command = Command::new(input_fin).args(arguments).spawn();

                match new_command {
                    Ok(mut new_command) => {
                        new_command.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }

    //Himnish => cd, exit, printf
    //Labdhi => bind
    //Sumeet => echo
}
