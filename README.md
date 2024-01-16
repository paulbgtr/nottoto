# nottoto

> A simple yet powerful tool for managing your notes efficiently. 

## Overview 

This application is designed with a robust Phoenix (Elixir) backend and a user-friendly Rust-based CLI frontend, offering a seamless experience for creating, editing, and organizing your notes.

## Features

- Create, Edit, and Delete Notes: Easily manage your notes with intuitive CLI commands.
- Search Functionality: Quickly find notes with a powerful search feature.

## Installation

### Backend

- Setup a PostgreSQL database with tables which definitions you may find in the `/backend/db/migrations.sql`.
- Setup the backend by running:

```bash
cd backend
npm i
nodemon server.js
```

### Frontend

- Compile the app and launch it using the following commands:

```bash
cd frontend
cargo run build --release
./target/release/frontend
```

#### Adding the program to PATH (doing so that you could use the program everywhere in your terminal)

This is done by adding the program to your path.

- First, you may want to rename the file to something more meaningful. For example `ntt`:

```bash
cd /frontend/target/release
mv frontend ntt
```

- Then you should add the binary to your PATH. You can use [this page](https://askubuntu.com/questions/109381/how-to-add-path-of-a-program-to-path-environment-variable) as a reference.

## Usage

All of the app's functionality is pretty well documented by [clap](https://docs.rs/clap/latest/clap/) library.

You can view all the commands by using the `-h` flag.
