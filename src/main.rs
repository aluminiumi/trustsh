use std::io;
use std::io::stdout;
use std::io::Write;

fn main() {
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
