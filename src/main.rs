use std::{
    env::{self},
    fs::File,
    io::{self, Read},
    path::Path,
    process::exit,
};

/// # Examples
/// ```
/// assert_eq!(2, count_trailing_spaces("  hi"));
/// assert_eq!(0, count_trailing_spaces("hi"));
/// assert_eq!(4, count_trailing_spaces("    hello world!"));
/// ```
fn count_trailing_spaces(string: &str) -> usize {
    let mut spaces: usize = 0;

    for character in string.chars() {
        if character == ' ' {
            spaces += 1;
        } else {
            return spaces;
        }
    }

    spaces
}

fn parse(contents: String) {
    // This holds all the SRC blocks, for example, if we parse the code below,
    // `srcs` will should be something like `fn main() {\n    println!("Hello world!")\n}`
    // ```
    // #+BEGIN_SRC
    //   fn main() {
    // #+END_SRC
    //
    // #+BEGIN_SRC
    //       println!("Hello world!");
    //   }
    // #+END_SRC
    // ```
    let mut srcs = String::new();

    // This keeps track of the SRC block that we are currently scanning, referring to the
    // code of the comment above, this will be first `fn main() {` and then
    // `    println!("Hello world!");\n}`
    let mut current_src = String::new();

    // These are used for...
    let mut lowest_space_level = 0; // ... counting how much trailing spaces we need to remove
    let mut is_first_src = true; // ... this is needed because we need to set `lowest_space_level`
                                 // to the first line of the first src, otherwise, if it
                                 // was just 0, we would remove 0 characters, doing nothing

    let mut is_src = false; // used to keep track wheter or not we are in an SRC block
    let mut i = 1; // used to keep track of the current line if any error comes up

    // TODO: Make this use less indentation
    // TODO: Add tests
    for line in contents.lines() {
        if line.trim().starts_with("#+BEGIN_SRC") {
            if is_src {
                eprintln!(
                    "Line {}: Cannot start an src without ending all of them first",
                    i
                );
                exit(1);
            }

            is_src = true;

            continue;
        } else if line.trim().starts_with("#+END_SRC") {
            if !is_src {
                eprintln!("Line {}: Cannot close an src without starting one", i);
                exit(1);
            }

            is_src = false;
            srcs.push_str(&current_src);
            current_src = String::new();

            continue;
        }

        if is_src {
            if is_first_src {
                is_first_src = false;
                lowest_space_level = count_trailing_spaces(line);
            } else if !line.is_empty() {
                lowest_space_level = lowest_space_level.min(count_trailing_spaces(line));
            }

            current_src.push_str(&format!("{}\n", line));
        }

        i += 1;
    }

    for line in srcs.lines() {
        if line.len() > 2 {
            println!("{}", &line[lowest_space_level..]);
        } else {
            println!("{}", line);
        }
    }
}

fn tangle_file(path: impl AsRef<Path>) {
    let mut path = match File::open(path) {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Couldn't find file");
            exit(1);
        }
    };

    let mut file_contents = String::new();
    if path.read_to_string(&mut file_contents).is_err() {
        eprintln!("Couldn't read file");
        exit(1);
    }

    parse(file_contents);
}

fn tangle_stdin() {
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).expect("Failed to read line") > 0 {}

    parse(input);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => tangle_stdin(),
        2 => tangle_file(&args[1]),
        _ => {
            eprintln!("Usage: `ballo /path/to/file.org` or `cat file.org | ballo`");
            exit(1);
        }
    }
}
