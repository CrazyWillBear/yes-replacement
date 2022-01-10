/*
  This software is licensed under the Unlicense. If a copy
  of said license wasn't provided with the code, you can go
  to https://unlicense.org/ to get one.

  Written by CrazyWillBear (https://crazywillbear.github.io),
  2021
*/

use std::env;
use std::io::stdout;
use std::io::Write;

// Software Version: '0.1.0'
const VER: &str = "0.1.0";

// usage function, sdf tring is formatted to represent text
// output in terminal.
fn usage() {
  eprintln!("
Usage: 'yes <input>...'
  or: 'yes OPTION'\n
Repeatedly prints input until killed.
  - Prints 'y' if input isn't provided.\n
Options:
  --help\tdisplay help menu
  --version\tdisplay version
  ")
}

fn main() {
  let args: Vec<String> = env::args().collect();

  // statement's default value is 'y'; will change later if
  // necessary
  let mut statement: String = "y".to_string();

  // if there are more args than just the program, user inputed
  // something, so assign statement whatever the input is
  if args.len() > 1 {
    // if user inputs '--help', run usage function and exit
    if args[1] == "--help" {
      usage();
      return;
    }
    // if user inputs '--version', display version
    if args[1] == "--version" {
      eprintln!("\nVersion = {}\n", VER);
      return;
    }
    // if the user inputs a '-<char>', and it failed the above
    // checks, the user put an unknown option, so throw error
    // and exit
    if args[1].split("").collect::<Vec<&str>>()[1] == "-" && args[1] != "-" {
      eprintln!("\nOption unknown '{}'\n\t- Use '--help' for more info\n", args[1]);
      return;
    }

    // create final statement
    statement = args[1..args.len()].join(" ");
  }

  // print statement forever (until killed)
  loop { stdout().write(format!("{}\n", statement).as_bytes()); }
}
