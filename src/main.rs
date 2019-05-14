use libc::{close, dup2, execvp, fork};
use std::env;
use std::io;
use std::io::{stdout, Write};

fn main() {
    // redirect stderr to stdout
    unsafe {
        dup2(libc::STDOUT_FILENO, libc::STDERR_FILENO);
    }

    // parse command line
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match &args[1][..] {
        "-h" => println!("help"),
        "-v" => println!("verbose"),
        "-p" => println!("prompt"),
        _ => println!("unknown argument"),
    }

    // proceed to infinite execution loop
    main_loop();
}

fn main_loop() {
    loop {
        // print prompt
        print!("$ ");
        stdout().flush()
            .expect("Failed to flush prompt");

        // feed user input into string
        let mut cmdline = String::new();
        io::stdin().read_line(&mut cmdline)
            .expect("Failure reading command line");

        // handle EOF/Ctrl-D
        if cmdline.len() == 0 {
            break;
        }

        eval(cmdline);
    }
}

fn eval(cmdline: String) {
    // repeat command to user
    println!("Command was {}", cmdline);

}
