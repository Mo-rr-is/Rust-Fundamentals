# Rust-Fundamentals
This is a Rust Programming Repo containing various example files that can help in learning Rust from basics.

Rust is a systems programming language that focuses on safety, performance, and concurrency.
- Macro: 
- Cargo: 
- Crate: 
- Dependencies: 
- 

## Hello World
### Example 1
As usual the first example is to print the Hello, World string using rust. This is done by first creating a main function, **fn main()**, that accepts no parameters. Inside the function, we can print the text using **println!()** command. Println! is a **macro** that prints text to the console. You can run the program and print the text in a command line.

That's how you make a simple hello world application using Rust Programming.

*Check hello-world folder*.

### Example 2
Next, we create a Hello, World application using a **Cargo**. Cargo is used to build applications and dependencies in Rust. In this case we use the command **cargo new hello-world-cargo --bin**. The bin at the end flags this as being an application and not a library. To run this cargo, simply go to the cargo directory and then use the command **cargo run**.

*Check hello-world-cargo folder*.

## Comments
Comments in Rust are done using "**//**" for line comments and "**/* ... */**" for block comments.

## Generating Random Numbers
You can use the **'rand'** crate which provides utilities for random number generation. A crate is a compilation unit and a packaging mechanism which serves as a fundamental building block for organizing and sharing code in Rust Projects. Crates can be libraries or binary executables.

You first add the **'rand'** crate as a dependency in your **'Cargo.toml'** file. In your Rust program, you first import the crate, then initialize the random number generator before then generating the random numbers.

*Check random_number_generator folder*