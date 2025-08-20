use assert_cmd::output;
use clap::{Arg, Command};
use std::path::Path;
use std::process;

mod compiler;
mod diagnostics;
mod utils;

use compiler::RusticCompiler;
use diagnostics::DiagnosticEngine;

fn main() {
    let _matches = Command::new("rustic")
        .version("0.10")
        .about("Rustic programming language compiler")
        .arg(
            Arg::new("input")
                .help("Input .rsc file or dir")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Ouput directory for generated rust code to native binary")
                .value_name("DIR")
                .default_value("target/generated")
        )
        .arg(
            Arg::new("compile")
                .short('c')
                .long("compile")
                .help("Compile generated Rust code to native binary")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input_path = _matches.get_one::<String>("input").unwrap();
    let output_dir = _matches.get_one::<String>("output").unwrap();
    let should_compile = _matches.get_flag("compile");
    let verbose = _matches.get_flag("verbose");

    let mut diagnostic_engine = DiagnosticEngine::new();
    let mut compiler = RusticCompiler::new(&mut diagnostic_engine);

    if verbose {
        println!("Rustic Compiler v0.1.0");
        println!("Input: {}", input_path);
        println!("Output: {}", output_dir);
    }

    let result = if Path::new(input_path).is_file() {
        compiler.compile_file(input_path, output_dir)
    } else {
        compiler.compile_directory(input_path, output_dir)
    };

    match result {
        Ok(generated_files) => {
            if verbose {
                println!("Generated {} Rust files:", generated_files.len());
                for file in &generated_files {
                    println!("  {}", file);
                }
            }

            if should_compile {
                match compiler.compile_to_native(&generated_files, output_dir) {
                    Ok(binary_path) => {
                        println!("Successfully compiled to: {}", binary_path);
                    }
                    Err(e) => {
                        eprint!("Compilation failed: {}", e);
                        process::exit(1);
                    }
                }
            }

            println!("Compilation successful!");
    }
    Err(e) => {
        eprint!("Error: {}", e);
        diagnostic_engine.emit_all();
        process::exit(1);
        }
    }
}