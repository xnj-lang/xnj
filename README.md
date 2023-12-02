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
    crt-int-var_name: 10; // Creating a variable "var_name" of int type, initialized with the value 10
    op("{var_name}"); // Output the variable "var_name"
    var_name++; // Increment "var_name" by 1
    opln("{var_name}"); // Output the updated value of "var_name" on a new line
    Ok() // Keyword to end the program
