# Raycasting in rust

Small [Raycasting](https://en.wikipedia.org/wiki/Ray_casting#Ray_casting_in_early_computer_games) implementation in Rust. The goal was to learn rust.

Try it out : https://albanesi.dev/demo/raycasting_demo/


## Compiling to web assembly

To compile and run the wasm version, you need to add the target and launch the script `compile_wasm.sh`

```
rustup target add wasm32-unknown-unknown
./compile_wash.sh
```

## Render

![](/assets/raycasting.png)