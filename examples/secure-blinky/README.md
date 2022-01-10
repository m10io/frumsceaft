# Secure Blinky

This sample contains an example bootloader / enclave built using Frumsceaft. It runs on an nRF5340. To build and flash you'll need the ARM GCC toolchain, and the nRF connect command line tools.

It is recomeneded that you use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), to make building easier. To build and flash using cargo-binutils run

```
cargo objcopy --target=thumbv8m.main-none-eabihf -- -O ihex boot.hex
nrfjprog --program boot.hex --sectorerase --reset
```

Once you have built the program, it is recomeneded to copy the `libnsclib.a` to the `non-secure-blinky` example, so it can find the NSC veneers.

```
cp (find . -name 'libnsclib.a') ../non-secure-blinky/libnsclib.a
```
