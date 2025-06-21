# Codex Apply Patch

A CLI tool and Python library for applying patches using a custom patch format designed for AI coding assistants.

This project is based on [OpenAI's original codex apply-patch tool](https://github.com/openai/codex/tree/main/codex-rs/apply-patch) and extends it with additional functionality.

## Changes from Original

This fork adds the following enhancements to the original OpenAI tool:

1. **In-Memory Patch Application**: New functionality to apply patches to files in memory without touching the filesystem, useful for testing and preview scenarios.

2. **Python Library Support**: Complete Python bindings using PyO3, allowing the patch functionality to be used directly from Python code.

3. **PyPI Distribution**: The library is packaged and distributed via PyPI for easy installation.

## Installation

### Python Package

```bash
pip install codex-apply-patch
```

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/openai/codex-apply-patch
cd codex-apply-patch
cargo build --release

# For Python development
pip install maturin
maturin develop --release
```

### Rust via GitHub

Add this to your `Cargo.toml`:

```toml
codex-apply-patch = { git = "https://github.com/nikhil-pandey/codex_apply_patch" }
```

## Usage

### Command Line

```bash
# Apply patch from stdin
echo "*** Begin Patch
*** Add File: hello.txt
+Hello, World!
*** End Patch" | codex_apply_patch
```

### Python Library

```python
import codex_apply_patch as cap

# Apply patch to files on disk
patch = """*** Begin Patch
*** Add File: hello.py
+print("Hello, World!")
*** End Patch"""

result = cap.apply_patch(patch)
print(result)

# Apply patch in memory (new feature)
files = {
    "main.py": "def main():\n    print('old version')\n"
}

patch = """*** Begin Patch
*** Update File: main.py
@@
 def main():
-    print('old version')
+    print('new version')
*** End Patch"""

result = cap.apply_patch_in_memory(patch, files)
print("Modified files:", result.files)
print("Summary:", f"Added: {len(result.added)}, Modified: {len(result.modified)}, Deleted: {len(result.deleted)}")

# Generate patch from file contents (new feature)
original = "def hello():\n    print('old version')"
new = "def hello():\n    print('new version')\n    print('updated!')"
patch = cap.generate_patch("main.py", original, new)
print("Generated patch:")
print(patch)

# Generate patch for multiple files (new feature)
file_changes = {
    "new.py": (None, "print('new file')"),  # Add file
    "old.py": ("old content", None),        # Delete file
    "update.py": ("old", "new")             # Update file
}
multi_patch = cap.generate_patch_from_files(file_changes)
print("Multi-file patch:")
print(multi_patch)
```

### Rust Library

#### Apply patch to disk

```rust
use codex_apply_patch::apply_patch;

let patch = r#"*** Begin Patch
*** Add File: hello.txt
+Hello from Rust!
*** End Patch"#;

apply_patch(patch, &mut std::io::stdout(), &mut std::io::stderr())?;
```

#### Apply patch in memory

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use codex_apply_patch::apply_patch_in_memory;

let files = HashMap::from([(PathBuf::from("hello.txt"), "old text\n".to_string())]);
let mut out = Vec::new();
let mut err = Vec::new();
let patch = r#"*** Begin Patch
*** Update File: hello.txt
@@
-old text
+new text
*** End Patch"#;
let result = apply_patch_in_memory(patch, &files, &mut out, &mut err)?;
```

#### Generate and parse patches

```rust
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use codex_apply_patch::{generate_patch, generate_patch_from_files, parse_patch};

let patch = generate_patch(Path::new("hello.txt"), None, Some("Hello\n"))?;
let hunks = parse_patch(&patch)?;

let changes = HashMap::from([(PathBuf::from("hello.txt"), (None, Some("Hello\n".to_string())))]);
let multi_patch = generate_patch_from_files(&changes)?;
```

#### Tool instructions

```rust
use codex_apply_patch::{APPLY_PATCH_TOOL_INSTRUCTIONS, APPLY_PATCH_API_INSTRUCTIONS};

println!("{}", APPLY_PATCH_TOOL_INSTRUCTIONS);
println!("{}", APPLY_PATCH_API_INSTRUCTIONS);
```

### Rust API Reference

- `apply_patch(patch, stdout, stderr)` - Apply patch to files on disk
- `apply_patch_in_memory(patch, files, stdout, stderr)` - Apply patch to in-memory files
- `parse_patch(patch)` - Parse patch and return hunks
- `generate_patch(path, original, new)` - Generate patch for a single file
- `generate_patch_from_files(changes)` - Generate patch for multiple files
- `APPLY_PATCH_TOOL_INSTRUCTIONS` - CLI tool usage text
- `APPLY_PATCH_API_INSTRUCTIONS` - Patch format instructions
### Python API Reference

- `apply_patch(patch_str)` - Apply patch to files on disk
- `apply_patch_in_memory(patch_str, files_dict)` - Apply patch to in-memory files 
- `parse_patch(patch_str)` - Parse patch and return structure information
- `generate_patch(path, original_content, new_content)` - Generate patch for a single file
- `generate_patch_from_files(files_dict)` - Generate patch for multiple files
- `get_tool_instructions()` - Get the CLI tool instructions for AI assistants
- `get_api_instructions()` - Get the patch format instructions (without CLI specifics)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

This project extends the original OpenAI codex apply-patch tool, which is also licensed under Apache-2.0.
