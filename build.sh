set -e;

# build it
cargo build -r;

# release it
cp ./target/release/adam ../../Gms2/SwordAndField/tools/bin/adam;
chmod 755 "./../../Gms2/SwordAndField/tools/bin/adam";