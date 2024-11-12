use properties_file_parser;
use properties_file_parser::parse_properties_as_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4{
        help(args[0].as_str(), false);
    }
    else if args[1].as_str() == "help" {
        help(args[0].as_str(), true);
    }
    else if args[1].as_str() == "parse" && args.len() > 2 {
        let unparsed = match std::fs::read_to_string(args[2].as_str()) {
            Ok(i) => i,
            Err(err) => {
                println!("File reading failed, error:");
                println!("{:?}", err);
                return;
            }
        };
        let parsed = parse_properties_as_string(unparsed.as_str());
        match parsed {
            Ok(parsed_str) => {
                if args.len() == 3 {
                    println!("{}", parsed_str.as_str());
                }
                else {
                    match std::fs::write(args[3].as_str(), parsed_str){
                        Ok(_) => {
                            println!("Result was written to {}", args[3].as_str());
                        }
                        Err(err) => {
                            println!("Writing result to file failed, error:");
                            println!("{:?}", err);
                            return;
                        }
                    }
                }
            }
            Err(err) => {
                println!("File parsing failed, error:");
                println!("{:?}", err);
                return;
            }
        }
    }
    else if args[1].as_str() == "credits" {
        println!("[--Credits:--]");
        println!("  Created by Andrusenko Mykola");
    }
    else {
        help(args[0].as_str(), false);
    }
}

fn help(command: &str, full_help: bool) {
    if full_help{
        println!("[--Help:--]");
        println!("  {} parse input_file", command);
        println!("      Parses .property file and prints the result.");
        println!("  {} parse input_file output_file", command);
        println!("      Parses .property file from input_file and stores it in output_file.");
        println!("  {} help", command);
        println!("      Shows the syntax of all commands.");
        println!("  {} credits", command);
        println!("      Shows credits.");
    }
    else {
        println!("[--Syntax:--]");
        println!("  {} parse input_file", command);
        println!("OR");
        println!("  {} parse input_file output_file", command);
        println!("  To get help, type {} help", command)
    }
}