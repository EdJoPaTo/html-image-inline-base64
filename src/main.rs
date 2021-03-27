use std::fs;

mod cli;
mod lib;

fn main() {
    let matches = cli::build().get_matches();

    let html_path = matches
        .value_of("html path")
        .expect("could not read html file path from command line");

    let output_path = matches.value_of("output path");

    let html_content = fs::read_to_string(html_path).expect("failed to read html file");

    let new_content = lib::html_inline(&html_content).expect("failed to inline external resources");

    if let Some(output_path) = output_path {
        fs::write(output_path, new_content).expect("failed to write output html to file");
    } else {
        println!("{}", new_content);
    }
}
