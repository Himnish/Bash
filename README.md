# Bash
## Group Name:   Bash
### Members:
### -Himnish Jain     (himnish2)
### -Labdhi Jain      (ljain2)
### -Sumeet Kulkarni  (sumeetk2)
Creating a Bash in Rust

### Introduction:

A Kernel is a computer program at the core of a computer's operating system. It has complete control over everything in the system, and is the "portion of the operating system code that is always resident in memory". The critical code of the kernel is usually loaded into a separate area of memory, which is protected from access by application software or other less critical parts of the operating system.

Kernels perform tasks such as running processes, managing hardware devices, and handling interrupts.

The kernel allocates requests from applications to perform I/O to an appropriate device.


### Goals and Objectives:

1. Write and create our own Shell/Kernel and run it inside our normal terminal emulator (wherever 
```cargo run```
 is typed).
2. Perform all the basic built ins, other commands and read arguments as done in a shell, such as 
```cd```
```ls```
```ls -a```

 etc. 
3. Handle errors properly, including the functionality for the 
```exit```
 command.
4. Possibly add a bootable image, so that the shell can run at boot.
5. Include functionality for pipes and IO redirection, so that the shell works properly
6. Simple Signal Handling
7. Custom Parser
8. Custom commands which would differ from normal shell commands


### Technical Components:

Here is a list of techincal components we are planning to use to complete this project:

- Pipes : Pipes are an interprocess communication mechanism that is provided in all flavors of Unix. A pipe is a one-way flow of data between processes: all data written by a process to the pipe is routed by the kernel to another process, which can thus read it.

 - I/O Redirection : Redirection can be defined as changing the way from where commands read input to where commands sends output.
 - Build standard feature of cargo : The build standard feature is required to be able to make the program be compatible for other platforms as well



### Possible challenges:

Some challenges we may face include figuring out how to use the “clap” crate in our project to parse terminal commands. GNU bash is a UNIX shell itself so we may face some issues parsing UNIX commands to Rust. Creating a kernel in Rust will also prove to be difficult, as we haven’t learned in class how exactly to create a bootable disk from the Rust binary.

Loading the code onto a system will also prove to be a challenge because it will require the project to work across operating systems. Coding the UNIX commands will be very advanced as well, and we may not have enough time for them in the end because coding the kernel is difficult enough. Simply put, coding an entire shell in Rust is a difficult task and we haven’t necessarily learned all the required knowledge to do this in class.



### References:

https://os.phil-opp.com/minimal-rust-kernel/
https://os.phil-opp.com/
https://www.gnu.org/software/bash/manual/html_node/index.html#SEC_Contents
https://github.com/clap-rs/clap
