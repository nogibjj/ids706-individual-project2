# ids706-individual-project2 [![CI](https://github.com/nogibjj/ids706-individual-project2/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/ids706-individual-project2/actions/workflows/rust.yml)


Individual Project #2: Rust CLI Binary with SQLite

## Requirements:

Your project should include the following:
- Rust source code: The code should comprehensively understand Rust's syntax and unique features.
- Use of GitHub Copilot: In your README, explain how you utilized GitHub Copilot in your coding process.
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.
- README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how GitHub Copilot was used.
- GitHub Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

## Grading Rubric for Project #2: Rust CLI Binary with SQLite

- Rust Source Code (25 points): Your Rust source code is well-structured and demonstrates a clear understanding of Rust's syntax and unique features.

	-	Proper usage of Rust syntax: 8 points
	-	Effective error handling in Rust: 8 points
	-	Implementation of Rust's unique features: 9 points

- SQLite Database (25 points): Demonstrates CRUD operations on the SQLite database.
	-	Create Operation: 6 points
	-	Read Operation: 6 points
	-	Update Operation: 6 points
	-	Delete Operation: 7 points


- Use of GitHub Copilot (10 points):

    - Explanation of the project: 3 points
    - How to run the program: 3 points
    - Dependencies and how to install them: 4 points

- Optimized Rust Binary (10 points): Process included that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.


- README.md (10 points): The README.md file is clear and concise and guides the user on how to run the program.
	-	Explanation of the project: 3 points
	-	How to run the program: 3 points
	-	Dependencies and how to install them: 4 points

- GitHub Actions (10 points): Your GitHub Actions file should test, build, and lint your Rust code correctly.
	-	Correct testing of Rust code: 3 points
	-	Correct building of Rust code: 3 points
	-	Correct linting of Rust code: 4 points

- Demo Video (10 points): A 2-5 minute video explaining the project and demonstrating its functionality is included. The video should be high-quality (both audio and visual), not exceed the given time limit, and be linked in the README via a private or public YouTube link.
	-	Clarity of explanation: 3 points
	-	Quality demonstration of the project: 3 points
	-	Quality of video and audio: 4 points

- Total: 100 points

## Project Explanation

This is a simple command-line interface (CLI) program written in Rust, designed to perform CRUD (Create, Read, Update, Delete) operations on a SQLite database. The primary objective of this project is to showcase the capabilities of Rust combined with interactions with the SQLite database.

With this CLI tool, users can effortlessly add, view, update, and delete user information from the database. The program provides an intuitive interactive interface, making database operations straightforward.

## Features
- Add User: Input name, age, and address to save new user data to the database.
- List All Users: Display detailed information about all users in the database.
- Update User: Modify the name, age, or address of a selected user.
- Delete User: Remove a selected user from the database.


## Using GitHub Copilot

In this project, I used GitHub Copilot to assist with programming. Here's how I use it:

1. **Code Suggestions**: Copilot provides me with instant code suggestions while writing code, which is especially helpful when writing CRUD operations.
2. **Error Checking**: Copilot helped me identify and fix some potential errors and poor coding practices.
3. **Code Optimization**: In some cases, Copilot suggests more optimized code solutions, which helps improve code quality and execution efficiency.

Overall, GitHub Copilot has greatly accelerated my programming speed and helped me improve the quality of my code.


## Optimize Rust binaries
In order to produce an optimized Rust binary, you should use the --release flag when building:

`cargo build --release`

This command will generate an optimized binary file in the target/release/ directory.


## Rust SQLite CLI

A simple command line interface program for CRUD operations on SQLite databases.

## Dependencies

- Rust: The project is written in the Rust programming language.
- SQLite: This is the lightweight database used in the project.
- rusqlite crate: For interacting with the SQLite database in Rust.
- Rust
	`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- SQLite
	`sudo apt-get install sqlite3 libsqlite3-dev`

## How to run

1. Clone this repository.
2. Run `cargo build --release` in the project directory.
3. Or Run `cargo build`.
4. Run `cargo run`.

## Video demonstration

[https://youtu.be/epv9oObsaVQ]