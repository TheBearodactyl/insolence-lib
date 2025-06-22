cross build --target x86_64-unknown-linux-gnu --release
cargo build --release

mkdir lib
Copy-Item "./target/x86_64-unknown-linux-gnu/release/liblibinsolence.so" "./lib/libinsolence.so"
Copy-Item "./target/release/libinsolence.dll" "./lib/libinsolence.dll"
