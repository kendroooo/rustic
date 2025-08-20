use crate::diagnostics::{self, DiagnosticEngine, Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use assert_cmd::output;
use walkdir::WalkDir;

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod semantic;

use ast::Program;
use codegen::CodeGenerator;
use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;

pub struct RusticCompiler<'a> {
    diagnostics: &'a mut DiagnosticEngine,
    modules: HashMap<String, Program>,
}

impl <'a> RusticCompiler<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine) -> Self {
        Self {
            diagnostics,
            modules: HashMap::new()
        }
    }

    pub fn compile_file(&mut self, input_path: &str, output_dir: &str) -> Result<Vec<String>> {
        let source = fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(format!("Failed to read file {}: {}", input_path, e)))?;

        let module_name = Path::new(input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main")
            .to_string();

        self.compile_source(&source, &module_name, input_path, output_dir)
            .map(|rust_file| vec![rust_file])
    }

    pub fn compile_directory(&mut self, input_dir: &str, output_dir: &str) -> Result<Vec<String>> {
        let mut generated_files = Vec::new();

        for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rsc") {
                let source = fs::read_to_string(path).map_err(|e| {
                    Error::IoError(format!("Failed to read file {:?}: {}", path, e))
                })?;

                let module_name = path
                     .file_stem()
                     .and_then(|s| s.to_str())
                     .unwrap_or("unnamed")
                     .to_string();

                let rust_file = self.compile_source(
                    &source,
                    &module_name,
                    path.to_str().unwrap_or(""),
                    output_dir,
                )?;
                generated_files.push(rust_file);
            }
        }

        Ok(generated_files)
    }

    fn compile_source(
        &mut self,
        source: &str,
        module_name: &str,
        file_path: &str,
        output_dir: &str,
    ) -> Result<String> {
        let mut lexer = Lexer::new(source, file_path);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens, self.diagnostics);
        let ast = parser.parse()?;

        let mut analyzer = SemanticAnalyzer::new(self.diagnostics);
        analyzer.analyze(&ast)?;

        self.modules.insert(module_name.to_string(), ast.clone());

        let mut codegen = CodeGenerator::new();
        let rust_code = codegen.generate(&ast, module_name)?;

        fs::create_dir_all(output_dir)
            .map_err(|e| Error::IoError(format!("Failed to create output directory: {}", e)))?;

        let rust_file_path = format!("{}/{}.rs", output_dir, module_name);
        fs::write(&rust_file_path, rust_code)
            .map_err(|e| Error::IoError(format!("Failed to write Rust file: {}", e)))?;

        Ok(rust_file_path)
    }

    pub fn compile_to_native(
        &self,
        rust_files: &[String],
        output_dir: &str,
    ) -> Result<String> {
        let cargo_toml = r#"[package]
name = "rustic-generated"
version = "0.1.0"
edition = "2021"

[dependencies]
rustic-runtime = { path = "../../../stdlib-runtime" }
"#;
        let cargo_path = format!("{}/Cargo.toml", output_dir);
        fs::write(&cargo_path, cargo_toml)
            .map_err(|e| Error::IoError(format!("Failed to write Cargo.toml: {}", e)))?;

        let lib_content = rust_files
            .iter()
            .map(|path|{
                let module_name = Path::new(path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed");
                format!("pub mod: {};", module_name)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let lib_path = format!("{}/lib.rs", output_dir);
        fs::write(&lib_path, lib_content)
            .map_err(|e| Error::IoError(format!("Failed to write lib.rs: {}", e)))?;

        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(output_dir)
            .output()
            .map_err(|e| Error::CompilationError(format!("Failed to run cargo: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf16_lossy(&output.stderr);
            return Err(Error::CompilationError(format!(
                "Cargo compilation failed: \n{}",
                stderr
            )));
        }

        let binary_path = format!("{}/target/release/rustic-generated", output_dir);
        Ok(binary_path)
        }
}
