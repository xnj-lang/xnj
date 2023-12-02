# XNJ Compiler

XNJ Compiler is a simple compiler written in Rust that compiles files with the `.xnj` extension.

## Getting Started

To use the compiler, follow these steps:

1. **Download the Executable:**
   - Download the executable file from the [Releases](https://github.com/xinajjj/xnj-compiler/releases) page.

2. **Set Up Environment:**
   - Open the command line.
   - Navigate to the directory containing the executable or add the compiler to the system's PATH.

3. **Compile `.xnj` File:**
   - Navigate to the directory containing your `main.xnj` file.
   - Run the compiler using the command:
     ```sh
     xnjc main.xnj
     ```

## Example Code

`main.xnj`:
```rust
main[Status]
    op("Hello World");
    Ok()
