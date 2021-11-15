use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
fn main() {
    loop {
        print!("🌽 > ");
        stdout().flush();

        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();
        let mut sections = inp.trim().split_whitespace();
        let input_fin = sections.next().unwrap();
        let mut arguments = sections;

        match input_fin {
            "cd" => {
                let new_dir = arguments.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            "echo" => {
                let mut output = String::new();
                let mut count = 0;
                let mut arg = "";
                for x in arguments {
                    if count==0&&(x=="-n"||x=="-e"||x=="-E") {
                        arg = x;
                        count = count + 1;
                        continue;
                    }
                    match arg {
                        "-e" => {
                            output.push_str(&x);
                            output.push_str(" ");
                        },
                        _ => {
                            let uninterpret_backslash = str::replace(x, "\\", "\\\\");
                            output.push_str(&uninterpret_backslash);
                            output.push_str(" ");
                        }

                    }
                    count = count + 1;
                }
                if arg=="-n" {
                    print!("{}", output.trim());
                } else {
                    println!("{}", output.trim());
                }
            }
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
