// Cli (Command Line Interface)

pub fn run() {
    // A variable that contains the arguments in a vector
    let args: Vec<String> = std::env::args().collect();
    let mut command = args[1].clone();
    command = command.to_lowercase();

    // echo command. Does the same thing as the echo built into windows and linux
    if command == "echo" {
        let echo = &args[2..args.len()].join(" ");
        println!("{}", echo);
    } else if command == "pwd" {
        // List the current directory/path and join it with "" to turn it into a string
        let path = std::env::current_dir().unwrap().join("");
        println!("{}", path.display());
    } else {
        println!("That is not a valid command.")
    }
}