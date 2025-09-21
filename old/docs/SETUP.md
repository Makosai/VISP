# Setup

> THIS IS DEPRECATED AS WE MOVED TO TAURI


Here's how to get your environment setup with the least amount of headaches.

## Rust

Install Rust. This project runs on 1.85.0.

## Dioxus and DX CLI

Dioxus has a [Getting Started](https://dioxuslabs.com/learn/0.6/getting_started/) guide that helps you out here.

Here's the easiest way:

```sh
cargo install cargo-binstall
```

```sh
cargo binstall dioxus-cli
```

And then make sure you have your platform specific dependencies like the guide tells you too.

## Tailwind

Dioxus has a guide helping you get started wtih [Setting up Tailwind](https://dioxuslabs.com/learn/0.6/cookbook/tailwind).

We've made it easier by including a `package.json`.

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

```sh
npm install
```

# Running VISP

This project includes basic organization with an organized `assets`, `components`, and `routes` folder.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving

You can run the script in the `scripts` folder to serve the application. Or...

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```
