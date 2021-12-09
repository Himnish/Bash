
use std::env;
use std::thread;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Child, Stdio};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::str::SplitWhitespace;

fn main() {
    let mut binds: HashMap<String, String> = HashMap::new();
    print!("\n🌽\n🌽 Welcome to the Corn-el 🌽\n🌽\n");
    loop {
        print!("🌽$ ");
        stdout().flush();

        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();

        

        let mut comm = inp.trim().split(" | ").peekable();

        for i in inp.trim().split(" | "){
            println!("{}", i);
        }
        // println!("{:?}", inp.trim().split(" | ").as_str());
        let mut prev_comm = None;

        while let Some(command) = comm.next() {

            let mut sections = inp.trim().split_whitespace();
            let mut input_fin = sections.next().unwrap();
            let mut arguments = sections;

            let mut sections = inp.trim().split_whitespace();

        let mut sections_back = inp.trim().split_whitespace();
        let mut final_input = sections_back.next_back().unwrap();  
        let mut run_background:bool = false;
        if(final_input == "&"){
            run_background = true;
        }
        let mut input_fin = sections.next().unwrap();
        let mut arguments = sections;  

        // for (key, val) in &binds{
        //     println!("{}, and val: {}", key, val);
        // }
            
            if binds.contains_key(input_fin){
                input_fin = &binds[input_fin];
            }

            match input_fin {
                "cd" => {
                    let new_dir = arguments.peekable().peek().map_or("/", |x| *x);
                    let base = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&base) {
                        eprintln!("{}", e);
                    }
                    prev_comm = None;
                },

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
                                let interpret_backslash = str::replace(x, "\\\\", "\\");
                                output.push_str(&interpret_backslash);
                                output.push_str(" ");
                            },
                            _ => {
                                output.push_str(&x);
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
                "bind" => {
                    let strin:String = arguments.next().unwrap().to_string();
                    let mut split:Vec<&str> = strin.split(":").collect();
                    let mut t1 = String::from(split[0]);
                    t1.retain(|c| c != '"' || c != '\'');
                    let mut t2 = String::from(split[1]);
                    t2.retain(|c| c != '"' || c != '\'');
                    &binds.insert(t1,t2);
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
                input_fin => {
                    let stdin = prev_comm.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
                    let stdout = if comm.peek().is_some() {
                        Stdio::piped()
                    } 
                    else {
                        Stdio::inherit()
                    };
                    let new_command = Command::new(input_fin).args(arguments).stdin(stdin).stdout(stdout).spawn();
                    match new_command {
                        Ok(new_command) => {
                            prev_comm = Some(new_command);
                        },
                        Err(e) => {
                            prev_comm = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
        if let Some(mut fin_comm) = prev_comm { fin_comm.wait().unwrap(); }
    }
}

// things that work:
// cd, echo, bind, exit, clear, ls
//Himnish => cd, exit, pipes
//Labdhi => bind
//Sumeet => echo
//pipes
