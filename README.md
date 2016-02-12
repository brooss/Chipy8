# Chipy8
A CHIP-8 emulator written in Rust using SDL2

![chipy8](chipy8.png)

#Compile
````
cargo build --release
````
#Run
````
./target/chipy8 ROMFILE
````
Or with cargo
````
cargo run --release -- ROMFILE
````
#Key Mappings
3 Different key mappings are provided (Default, Alt and Tetris). Selected with F1, F2 and F3 keys. All mappings map to WASD+Space keys. Period key '.' resets the emulator. Esc exits.
#Example
To compile and run with the included Tetris
````
cargo run --release -- ./roms/TETRIS
````
Then change to the Tetris key mapping with F1 key.
