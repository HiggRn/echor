use clap::{arg, command};

fn main() {
    let matches = command!()
        .args(&[
            arg!(text: <TEXT> "Input text")
                .num_args(1..),
            arg!(omit_newline: -n "Do not print newline")
        ])
        .get_matches();
    
    let text: Vec<String> = matches.get_many::<String>("text")
        .unwrap()
        .map(String::clone)
        .collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"})
}
