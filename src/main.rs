use std::env::args;
use std::process;
use std::fs;

fn process_file(file: String) -> String {
    let file = fs::read_to_string(file)
        .expect("Couldn't read file! Does it exist!");
    return file
}

fn main() {
    if args().len() != 1 || args().len() > 1 {
        println!("You didn't use correct arguments!");
        println!("Usage: metalcat.exe {{filename}} (without brackets)");
        process::exit(0)
    }

    let args: Vec<String> = args().collect();
    let argument = &args[1];
    println!("{}", process_file(argument.to_string()))

}
