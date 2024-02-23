use colored::Colorize;

pub struct ReadDirOptions {
    // Show files beginning with a '.'
    pub dotfiles: bool,

    // Show '.' and '..' (overriden by `dotfiles`)
    pub implied: bool,

    // Should the files be sorted
    pub sort: bool,

    // Reverse the finalized vector
    pub reverse: bool,
}

impl ReadDirOptions {
    pub fn new() -> Self {
        Self {
            dotfiles: false,
            implied: false,
            sort: true,
            reverse: false,
        }
    }
}

pub fn read_dir_to_string(options: ReadDirOptions) -> String {
    let mut paths: Vec<String> = std::fs::read_dir("./")
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap())
        .collect();
    
    if options.implied {
        paths.push(String::from("."));
        paths.push(String::from(".."));
    }

    if !options.dotfiles{
        paths = paths
            .into_iter()
            .filter(|path| path.as_bytes()[0] != b'.')
            .collect();
    }

    if options.sort {
        paths.sort();
    }

    if options.reverse {
        paths.reverse();
    }

    // Colorize directories
    for path in &mut paths {
        if std::path::Path::new(&path).is_dir() {
             *path = path.bold().blue().to_string();
        }     
    }

    paths.join(" ")
}
