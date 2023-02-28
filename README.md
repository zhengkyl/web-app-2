# web app 2

This is a web app created with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Run locally

```sh
npx tailwindcss -i ./style/input.css -o ./style/output.css --watch

cargo leptos watch
```

## Styling

Put tailwind code in `input.css`. Then, `cargo-leptos` optimizes the source css file specified in `Cargo.toml` which is `output.css`.

### Resources

https://tailwind-elements.com

## Setup

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)
