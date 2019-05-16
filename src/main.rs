use libc::{dup2}; // close, execvp, fork
use std::env;
use std::io;
use std::io::{stdout, Write};
use std::process;

const EXIT_ERROR: i32 = 1;
const MAX_ARGS: usize = 255;

enum BuiltinState {
    None,
    Quit,
    Jobs,
    Bg,
    Fg,
}

enum ParseStatus {
    ParselineFg,
    ParselineBg,
    ParselineEmpty,
    ParselineError,
}

struct CmdlineTokens<'a> {
    argc: usize,
    //argv: Option<[&'a str; MAX_ARGS]>,
    argv: [&'a str; MAX_ARGS],
    infile: Option<&'a str>,
    outfile: Option<&'a str>,
    builtin_state: BuiltinState,
}

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

fn main_loop<'a>(disable_prompt: bool, verbose: bool) {
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

    // parse command line into tokens
    // list of args, infile, outfile, bool_builtin
    let (parse_result, token) = parse_line(&cmdline);

    // if not builtin, handle external command
    //if token.builtin == None {
    //    execute(token, parse_result);
    // otherwise, handle builtin command
    //} else {
    //    //handle_builtin(token);
    //}
}

fn parse_line(cmdline: &String) -> (ParseStatus, CmdlineTokens) {
    println!("parse_line: {}", cmdline);

    let whitespace = "\r\n\t ";
    let mut status = ParseStatus::ParselineError;
    let mut token = CmdlineTokens {
        argc: 0,
        argv: [&cmdline[..]; MAX_ARGS],
        infile: None,
        outfile: None,
        builtin_state: BuiltinState::None,
    };

    // used by state machine to parse where string portions go
    enum ParseState {
        StateInfile,
        StateOutfile,
        StateDefault,
        StateError,
    }
   
    let mut last_char_was_ws = false;
    let mut startindex = 0;
    let mut fastfwd_to = 0;
    let mut state = ParseState::StateDefault;
    for (index, charb) in cmdline.chars().enumerate() {

        // if searching for matching quotation mark, skip everything
        if fastfwd_to > index {
            continue;
        }

        // if current char is a whitespace char
        if whitespace.contains(charb) {
            if !last_char_was_ws {
                match state {

                    // if last state was for infile/outfile
                    ParseState::StateInfile | ParseState::StateOutfile => {
                        if startindex == index {
                            // have not seen non-whitespace in this state yet
                            // so just keep going
                            startindex += 1;

                        } else {
                            // save the file name to struct
                            let slice = &cmdline[startindex..index];
                            match state {
                                ParseState::StateInfile => {
                                    match token.infile {
                                        None => token.infile = Some(slice),
                                        Some(_x) => {
                                            eprintln!("uh oh: multiple infiles specified");
                                            status = ParseStatus::ParselineError;
                                            break;
                                        }
                                    }
                                }

                                ParseState::StateOutfile => {
                                    match token.outfile {
                                        None => token.outfile = Some(slice),
                                        Some(_x) => {
                                            eprintln!("uh oh: multiple outfiles specified");
                                            status = ParseStatus::ParselineError;
                                            break;
                                        }
                                    }
                                }

                                _ => eprintln!("uh oh: impossible state"),
                            }

                        }
                    }

                    // if last state was default
                    ParseState::StateDefault => {
                        // save string token in struct
                        let slice = &cmdline[startindex..index];
                        token.argv[token.argc] = slice;
                        token.argc += 1;
                    }

                    _ => {
                        eprintln!("uh oh: error state reached");
                        break;
                    }
                }

                last_char_was_ws = true;
            }

            // we've handled the char match, so skip everything else this iteration
            continue;
            
        }

        // if current char is not whitespace
        if last_char_was_ws {
            startindex = index;
        }

        match charb {
            '<' => {
                println!("\nstate: leftarrow");
                state = ParseState::StateInfile;
                startindex = index+1;
            }

            '>' => {
                println!("\nstate: rightarrow");
                state = ParseState::StateOutfile;
                startindex = index+1;
            }

            '\'' | '\"' => {
                // find and jump to the next quotation mark
                let slice = &cmdline[index+1..];
                match slice.find(charb) {
                    Some(next_quote) => {
                        startindex = index;
                        fastfwd_to = next_quote+index+2;
                        last_char_was_ws = false;
                        continue;
                    }
                    None => {
                        // unmatching quote; fail
                        status = ParseStatus::ParselineError;
                        break;
                    }
                }
            }

            _ => {
                // non-special character
            }

        }

        last_char_was_ws = false;

        //print!("{}", charb);
        //stdout().flush()
        //    .expect("Failed to flush prompt");

    } // end of if current char not whitespace

    //println!("");

    // examine the first token to see if it's a builtin command
    match token.argv[0] {
        "quit" | "exit" => {
            token.builtin_state = BuiltinState::Quit;
        }
        "jobs" => {
            token.builtin_state = BuiltinState::Jobs;
        }
        "bg" => {
            token.builtin_state = BuiltinState::Bg;
        }
        "fg" => {
            token.builtin_state = BuiltinState::Fg;
        }
        _ => println!("something else"),
    }

    /*
    match token.builtin_state { 
        BuiltinState::None => println!("external command"),
        _ => println!("internal command"),
    }

    // print all tokens
    for x in 0..token.argc {
        println!("{}", token.argv[x]);
    }
    // print infile
    match token.infile {
        None => println!("infile: None"),
        Some(x) => println!("infile: {}", x),
    }
    // print outfile
    match token.outfile {
        None => println!("outfile: None"),
        Some(x) => println!("outfile: {}", x),
    }
    */

    (status, token)
}
