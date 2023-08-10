# Reverse.rs
### Description
A small command line program written in rust that reverses the sound in as `.wav` file. It was
inspired by harvard's [CS50x 2023 "Reverse" problem](https://cs50.harvard.edu/x/2023/psets/4/reverse/). 
It serves as a fun exploration for comparison with my C solution submitted for the problem. 

### Usage
Run the project with `cargo run`, you will need Rust's cargo installed. Please see the official documentation for how.
Included with the source code is `expected_output.wav` which can be used in crude testing of the program's output. 
It was the output given from my C program that was submitted and received top score. No output from `cmp output.wav expected_output.wav` in powershell, confirms that both files are identical and thus, this rust version works as intended.

### Further notes
I'm sure speed and robustness can be improved for this rust program and the C one. I am curious how they might compare in real-world performance and if it is possible to get unintended behaviour, I may do so at a later date. Any suggestions are welcome.
