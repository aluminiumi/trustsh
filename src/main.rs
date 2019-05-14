use libc::{close, dup2, execvp, fork};
use std::env;
use std::io;
use std::io::{stdout, Write};
use std::process;

const EXIT_ERROR: i32 = 1;

fn main() {
    // initialize variables
    let mut disable_prompt = false;
    let mut verbose = false;

    // redirect stderr to stdout
    unsafe {
        dup2(libc::STDOUT_FILENO, libc::STDERR_FILENO);
    }

    // parse command line
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() == 2 {
        match &args[1][..] {
            "-h" => usage(),
            "-v" => verbose = true,
            "-p" => disable_prompt = true,
            _ => usage(),
        }
    }

    // proceed to infinite execution loop
    main_loop(disable_prompt, verbose);
}

fn usage() {
    println!("Usage: trustsh [-hvp]\n");
    println!("   -h   prints this message\n");
    println!("   -v   prints additional debug information\n");
    println!("   -p   disable display of command prompt\n");
    process::exit(EXIT_ERROR);

}

fn main_loop(disable_prompt: bool, verbose: bool) {
    if verbose {
        println!("verbose mode enabled");
    }

    loop {
        // print prompt
        if !disable_prompt {
            print!("$ ");
            stdout().flush()
                .expect("Failed to flush prompt");
        }


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
