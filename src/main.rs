use std::collections::HashMap;
use std::env;
use std::thread;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime};
fn main(){
    let mut binds: HashMap<String, String> = HashMap::new();
    print!("\n🌽\n🌽 Welcome to the Corn-el 🌽\n🌽\n");
    loop {
        print!("🌽$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
<<<<<<< HEAD
            let command = parts.next().unwrap();
            let args = parts;
=======
            let mut command = parts.next().unwrap();
            let mut args = parts;
>>>>>>> fdd012ed8bbc84486a1a98e00b89e3ad354b1bb4

            if binds.contains_key(command){
                command = &binds[command];
            }

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let base = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&base) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                "exit" => return,
                "echo" => {
                    let mut output = String::new();
                    let mut count = 0;
                    let mut arg = "";
                    for x in args {
                        if count==0&&(x=="-n"||x=="-e"||x=="-E") {
                            arg = x;
                            count = count + 1;
                            continue;
                        }
                        match arg {
                            "-e" => {
                                let interpret_backslash = str::replace(x, "\\\\", "\\");
                                output.push_str(&interpret_backslash);
                                output.push_str(" ");
                            },
                            _ => {
                                let remove_quote = str::replace(x, "\"", "");
                                output.push_str(&remove_quote);
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
                },
                "times" => {
                    let now = SystemTime::now();
                    match now.elapsed() {
                        Ok(elapsed) => {    
                          println!("0m{}s 0m{}s", (elapsed.as_nanos() as f32)/(100000.0), (elapsed.as_nanos() as f32)/(100000.0));
                        }
                        Err(error) => {
                            println!("System time error");
                        }
                    }
                },
                "bind" => {
                    let strin:String = args.next().unwrap().to_string();
                    let mut split:Vec<&str> = strin.split(":").collect();
                    let mut t1 = String::from(split[0]);
                    t1.retain(|c| c != '"' || c != '\'');
                    let mut t2 = String::from(split[1]);
                    t2.retain(|c| c != '"' || c != '\'');
                    &binds.insert(t1,t2);
                },
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }

    }
}

// things that work:
// cd, echo, bind, exit, clear, ls
//Himnish => cd, exit, pipes
//Labdhi => bind
//Sumeet => echo
//pipes
