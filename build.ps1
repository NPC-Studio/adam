# build it
cargo build -r;

# release it
Move-Item `
-Path ./target/release/adam.exe `
-Destination ./../../Gms2/SwordAndField/tools/bin/adam.exe `
-Force;
