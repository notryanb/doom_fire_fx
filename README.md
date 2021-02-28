# Doom Fire FX

The "Doom fire effect" written about in my [blog post](http://notryanb.github.io/rust-doom-fire-fx.html),
which was originally implemented via SDL2 - commit #b2103ad (I didn't tag it.. :( ).

The most recent changes to this project include swapping out SDL2 bindings with the [Druid](https://github.com/linebender/druid) project which allows cross platform support with minimal setup.

- Works on Windows 10
- Works on MacOS Mojave
- Works compiled to wasm, but there are significant performance issues.

To run the wasm project, you must have [wasm-pack](https://github.com/rustwasm/wasm-pack) installed.
To serve the project, use a tool like [https](https://lib.rs/crates/https) to serve via `localhost:8000`. Running `http` from the root of the project after building the wasm should work.

# Running

## In development
- `cargo run` => Runs on your native system
- `wasm-pack build --target web --dev` => builds the wasm module

## In release mode (performance improvements)
- `cargo run --release` => Runs on your native system
- `wasm-pack build --target web` => builds the wasm module


# TODO
- Re-implement the "scroll down" effect which reveals an image behind the fire
- Investigate wasm performance issues. The druid examples for game of life and animation look to also suffer from performance issues, so it may not be directly related to the fire generation code, however there is definitely room for improvement.