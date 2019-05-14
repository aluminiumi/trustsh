use std::io;

fn main() {
    loop {
        // print prompt
        println!("$ ");

        let mut cmdline = String::new();
        io::stdin().read_line(&mut cmdline)
            .expect("Failure reading command line");

        println!("Command was {}", cmdline);
    }
}
