echo "Building for web assembly"
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/raycasting.wasm dist/raycasting.wasm
cd dist
echo "Starting web server at 127.0.0.1:8000"
python -m http.server --bind 127.0.0.1