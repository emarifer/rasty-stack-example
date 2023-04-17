# Example of using a full stack with Rust.

### This repository is an example of a complete stack built entirely in Rust ("RASTY"):

- [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language) as almost unique base language (except for minimal JS needed to style the interface with TailwindCSS)
- [Actix-web](https://actix.rs/), a Rust framework, for the backend (our API server)
- [SurrealDB](https://surrealdb.com/), the database that connects with our API, which has been developed by its creators with Rust
- [Tauri App](https://tauri.app/), a Rust framework for building desktop applications, using web technology for the user interface
- [Yew.rs](https://yew.rs/), Rust's web framework for building web applications with Webassembly

### In itself, the frontend applications perform a CRUD to the database using the RESTful API.

---

## To run the application stack

### 1.- actix-surrealdb-api, server/API built using Rust's Actix-web framework and SurrealDB database running in a Docker container.

In addition to the obvious prerequisite of having Rust and Docker installed, we need to do the following:

Create the container with a SurrealDB image and a Docker managed storage volume running at the project root:

```
$ docker compose up -d
```

If we have _cargo-watch_ installed using:

```
$ cargo install cargo-watch
```

we won't have to restart the server every time we make a code change; running the following command in the root of the project actix-surrealdb-api:

```
$ cargo watch -x run
```

rather:

```
$ cargo run
```

the server will restart automatically 😀.

To stop the Docker container in which SurrealDB is running:

```
$ docker stop surrealdb
```

And to raise the container again:

```
$ docker start surrealdb
```

We can read the content of the database with the commands:

```
$ docker exec -it surrealdb /surreal sql -c http://localhost:8000 -u root -p root --ns namespace --db database --pretty

namespace/database> SELECT * FROM task ORDER BY created_at DESC;
```

---

### 2.- todo-yew-web, Web application developed with Rust/WebAssembly + Yew + Tailwindcss.

To run the Web App, add the WebAssembly target:

```
$ rustup target add wasm32-unknown-unknown
```

Install [Trunk](https://trunkrs.dev/) (web application bundler for Rust) to run the app:

```
$ cargo install trunk
```

Run the following command in the root of the project todo-yew-web:

```
$ trunk serve
```

Build the application by running:

```
$ trunk build --release
```

### 3.- todo-tauri-desktop, Desktop application developed with Rust/WebAssembly + Yew + Tailwindcss.

Screenshot:

<img src="https://user-images.githubusercontent.com/68773736/232398437-827f6fb9-2b70-4f04-ad6f-d470c868bfb6.png" width="75%">

As prerequisites, in addition to the Rust language and some OS-dependent libraries required for Tauri, you must also install the build target for browser-based WebAssembly called "wasm32-unknown-unknown" and the "Trunk" tool to the deployment and packaging of the Yew frontend (seen in the previous section).

On the other hand, if we want to start from scratch, to create the scaffolding of the Tauri + Yew application it is necessary to install the Tauri app creation tool for the Cargo package manager and the Tauri CLI:

```
$ cargo install create-tauri-app && cargo install tauri-cli
```

Finally, since we use the Tailwind CSS framework, we will have to run in the root of the project todo-tauri-desktop (you need to have NodeJs installed):

```
$ npm i
```

With all this accomplished, run the app under development with the command:

```
$ cargo tauri dev
```

or build it with the command:

```
$ cargo tauri build
```

(for more information see the documentation of [Tauri](https://tauri.app/) and [Yew](https://yew.rs/).)

---

## Bonus: Frontend of the application developed with [SolidJS](https://www.solidjs.com/) (Web & Desktop).

### In the solidjs branch of the repository, the version of the stack that SolidJS uses in both the Web app frontend and the Desktop app frontend is available. Both use pnpm as bundler and dependency manager.

To add the dependencies run on todo-solidjs-web:

```
$ pnpm i
```

To run it in development mode:

```
$ pnpm run dev
```

To run it in development mode:

```
$ pnpm run build
```

In the case of the desktop application developed with Tauri + SolidJS, we run the same command to add the dependencies (pnpm i). To launch the application in development mode we will execute:

```
$ pnpm tauri dev
```

And finally, to create the executable binary on our platform, we run:

```
$ pnpm tauri build
```

[comment]: # "https://user-images.githubusercontent.com/68773736/232398437-827f6fb9-2b70-4f04-ad6f-d470c868bfb6.png"
