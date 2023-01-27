cargo bootimage --release
sudo dd if=target/x86_64-CrystalOS/release/bootimage-CrystalOS.bin of=/dev/sdc
