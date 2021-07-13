// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::env;


mod sat_solver;
mod implication_graph;
mod parser;
mod util;

use util::show_error;

use sat_solver::SatSolver;
use parser::parse_file;

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_error(&("no argument given. Exiting. ".to_string()));
        return;
    }

    let filename: &String = &args[1];


    let mut sat: SatSolver = parse_file(filename);
    sat.print();

    sat.solve();
    sat.print_solution();
    
    sat.check_solution();



}
