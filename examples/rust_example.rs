use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use codex_apply_patch::{
    APPLY_PATCH_API_INSTRUCTIONS, APPLY_PATCH_TOOL_INSTRUCTIONS, apply_patch,
    apply_patch_in_memory, generate_patch, generate_patch_from_files, parse_patch,
};
use tempfile::tempdir;

fn example_basic_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Usage ===");

    let patch =
        "*** Begin Patch\n*** Add File: hello.txt\n+Hello from Rust!\n*** End Patch";

    let dir = tempdir()?;
    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(&dir)?;
    apply_patch(patch, &mut std::io::stdout(), &mut std::io::stderr())?;
    std::env::set_current_dir(cwd)?;

    let content = fs::read_to_string(dir.path().join("hello.txt"))?;
    println!("Created file:\n{}", content);
    Ok(())
}

fn example_in_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== In-Memory Usage ===");

    let files =
        HashMap::from([(PathBuf::from("hello.txt"), "old text\n".to_string())]);
    let patch =
        "*** Begin Patch\n*** Update File: hello.txt\n@@\n-old text\n+new text\n*** End Patch";

    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = apply_patch_in_memory(patch, &files, &mut out, &mut err)?;
    for (p, c) in &result.files {
        println!("{} =>\n{}", p.display(), c);
    }
    Ok(())
}

fn example_generate_patch() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Generate Patch ===");

    let original = "fn greet() {\n    println!(\"old\");\n}\n";
    let new = "fn greet() {\n    println!(\"new\");\n}\n";
    let patch = generate_patch(Path::new("greet.rs"), Some(original), Some(new))?;
    println!("Patch for one file:\n{}", patch);

    let changes = HashMap::from([
        (
            PathBuf::from("new.rs"),
            (None, Some("fn new_file() {}\n".to_string())),
        ),
        (PathBuf::from("delete.rs"), (Some("old".to_string()), None)),
    ]);
    let multi = generate_patch_from_files(&changes)?;
    println!("Patch for multiple files:\n{}", multi);
    Ok(())
}

fn example_parse_patch() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Parse Patch ===");
    let patch = "*** Begin Patch\n*** Add File: foo.rs\n+fn foo() {}\n*** End Patch";
    let hunks = parse_patch(patch)?;
    println!("Parsed {} hunk(s)", hunks.len());
    Ok(())
}

fn example_instructions() {
    println!("\n=== Instructions ===");
    println!("Tool: {}", &APPLY_PATCH_TOOL_INSTRUCTIONS[..80]);
    println!("API: {}", &APPLY_PATCH_API_INSTRUCTIONS[..80]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_basic_usage()?;
    example_in_memory_usage()?;
    example_generate_patch()?;
    example_parse_patch()?;
    example_instructions();
    Ok(())
}
