use std::{collections::VecDeque, env};

use svm::{VMOptions, VirtualMachine};

fn main() {
    let mut args: VecDeque<_> = env::args().collect();

    // Remove binary name from argv
    args.pop_front();

    // Parse command line arguments
    let args = parse_args(args);

    println!("{args:#?}");

    let vm = VirtualMachine::new(args);

    println!("{vm:02X?}")
}

/**
 * Parses assembler arguments from command line argv
 */
fn parse_args(mut args: VecDeque<String>) -> VMOptions {
    let mut rom_binary: Option<String> = None;
    let mut debug: bool = false;

    if args.is_empty() {
        print_help_statement();
        std::process::exit(1);
    }

    while !args.is_empty() {
        // We know since the argv is not empty that we can unwrap
        let arg = args.pop_front().unwrap();

        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                println!("SPASM v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "-d" | "--debug" => {
                debug = true;
            }
            _ => {
                if arg.starts_with("-") {
                    eprintln!("Unexpected option argument '{arg}'!");
                    print_help_statement();
                    std::process::exit(1);
                } else if !args.is_empty() {
                    eprintln!("Unexpected arguments after file name: {:?}", args);
                    print_help_statement();
                    std::process::exit(1);
                }

                rom_binary = Some(arg);
            }
        }
    }

    let rom_binary = match rom_binary {
        Some(out) => out,
        None => {
            eprintln!("Expected file name after options!");
            print_help_statement();
            std::process::exit(1);
        }
    };

    if !rom_binary.ends_with(".bin") {
        eprintln!("File name '{rom_binary}' must end with '.bin'!");
        print_help_statement();
        std::process::exit(1);
    }

    VMOptions { rom_binary, debug }
}

/**
 * Print SPASM usage
 */
fn print_usage() {
    println!("      SVM - SIS16 Virtual Machine");
    println!("A fully featured SIS16 interpreter to substitute for real hardware");
    println!("");
    println!("Usage:");
    println!("  svm --help");
    println!("  svm --version");
    println!("  svm [options...] rom_binary");
    println!();
    println!("Options:");
    println!("  -h, --help                    Prints this help dialogue");
    println!("  -v, --version                 Print the current version");
    println!("  -d, --debug                   Emits debug extra information");
    println!();
    println!("Examples:");
    println!("  svm --debug main.bin");
}

fn print_help_statement() {
    println!("Use 'svm --help' to see usage!")
}
