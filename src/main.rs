use std::{
    env::{self, current_dir},
    fs::File,
    io::Read,
    process::exit,
};

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
            current_src.push_str(&format!("{}\n", line));
        }

        i += 1;
    }

    for line in srcs.lines() {
        println!("{}", line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: `tangler /path/to/file.org`");
        exit(1);
    }

    let mut path = match File::open(&args[1]) {
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
