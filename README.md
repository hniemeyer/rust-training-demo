# Basic Rust training source code

This repository contains examples for several basic rust topics as a workspace.

* geometry + application: Basic syntax and module system
* iterators: Writing your own iterator and using the iterator methods from stdlib and itertools
* patternmatching: Pattern matching and destructuring
* errorhandling: Error handling in Rust with the eyre crate
* macros: Example for macros that are not procedural macros
* multithreading: Using threads in Rust

## Usage if you dont have Rust installed

You dont need to have Rust installed. If you have Docker and Visual Studio Code
on your machine you can use the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extenstion from Microsoft.

There are two ways to use this extension with this repository:

Clone this repo to your disk, start VS Code, run the Remote-Containers: Open Folder in Container... command from the Command Palette (F1) or quick actions Status bar item, and select the folder where git put this repo (likely the name of the repo i.e. rust-training-demo).

OR

Start VS Code and run Remote-Containers: Clone Repository in Container Volume... from the Command Palette (F1). Enter <https://github.com/hniemeyer/rust-training-demo> in the input box that appears and press Enter.

This will start a docker container with the code, Rust and appropriate extensions already installed and will open a VS code instance which can interact with the container.

An alternative which does not run on your own computer is opening the repo in <https://gitpod.io>.

## Resources to learn Rust

* [Rustling exercixes](https://github.com/rust-lang/rustlings)
* [Rust book](https://doc.rust-lang.org/book/)
* [Rust by example](https://doc.rust-lang.org/rust-by-example/)
* [Rust cookbok](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
