# async-avr

## Initial installation
`async-avr` needs nightly rust:

```bash
rustup install nightly
```

## Compiling and Running

We can compile by running

```bash
cargo +nightly build -Z build-std=core --release --target avr-atmega328p.json
```

Then, to upload it to a device, enable "Show verbose output during: upload" in the Arduino IDE. Observe the build logs for an `avrdude` command—it should look something like:

```bash
/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude -C/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:/tmp/arduino_build_721874/Blink.ino.hex:i
```

Copy that command, but delete everything after `-Uflash:w:`. Then, without spaces, add the path to your binary. This will typically be `target/avr-atmega328p/release/project_name.elf`, or `target/avr-atmega328p/release/examples/example_name.elf`. Finally, add `:e`. Your final command will probably look something like:

```bash
/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude -C/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/example/serial.elf:e
```

> ### What about converting to hex first?
>
> Arduino typically converts the compiled binary to raw hex, and many AVR-Rust projects have [followed that pattern][avr-objcopy]. However, there's generally no need to do that, as `avrdude` has the ability to upload ELF binaries directly.
>
> [avr-objcopy]: https://github.com/Rahix/avr-hal/blob/bfc5dfe67107a68b4a673e54532354af126cb3ba/mkhex.sh#L32
