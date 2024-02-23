use rd::ReadDirOptions;

const HELP: &str = r#"
    USAGE: rd -[OPTIONS]... [FILE]
    DESCRIPTION: List files in the given directory

    ARGUMENTS:
        -a | -d => list dotfiles (files beginning with a '.')
        -A | -D => list dotfiles (files beginning with a '.') aswell as implied entries ('.' and '..') 
        -u => do not apply sorting
        -r => reverse the FINAL file vector
        -h => print this menu; exit
        -v => print the version; exit

    ERRORS: Upon an error the program will print the help menu then the error; after it will proceed to exit
        * If you add multiple dashes in an argument
        * If you add an invalid argument
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect(); 

    let mut read_dir_options = ReadDirOptions::new();

    let mut directory_arg: Option<String> = None;
    for mut arg in args {
        if arg.starts_with("-") {
            arg.remove(0);
            for char in arg.chars() {
                match char {
                    'a' => {
                        read_dir_options.dotfiles = true;    
                    } 

                    'A' => { 
                        read_dir_options.dotfiles = true;
                        read_dir_options.implied = true;
                    }

                    'u' => {
                        read_dir_options.sort = false;
                    }

                    'r' => {
                        read_dir_options.reverse = true;
                    }

                    'h' => {
                        println!("{}", HELP);
                        std::process::exit(0);
                    }

                    'v' => {
                        println!(env!("CARGO_PKG_VERSION"));
                        std::process::exit(0);
                    }

                    '-' => {
                        println!("{}", HELP);
                        eprintln!("Error: please include only one '-'; exiting");
                        std::process::exit(1);
                    }

                    _ => {
                        println!("{}", HELP);
                        eprintln!("Error: unknown argument; exiting");
                        std::process::exit(1);
                    } 
                }
            }
        } else {
            if directory_arg.is_none() {
                directory_arg = Some(arg);
            } else {
                eprintln!("Error: multiple directory arguments; exiting");
                std::process::exit(1);
            }
        }
    } 

    println!("{}", rd::read_dir_to_string(read_dir_options));
}
