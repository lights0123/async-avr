# async-avr

## Required packages

**Arch linux**

```bash
sudo pacman -S avr-gcc avr-libc avrdude python-pip
```

**Ubuntu**

```bash
sudo apt-get install binutils gcc-avr avr-libc avrdude python3-pip
```

**macOS**

```bash
brew tap osx-cross/avr
# removal needed before upgrading
brew remove avr-gcc avr-binutils avr-libc
# avr-libc is now included in avr-gcc
brew install avr-gcc avr-binutils
brew install avrdude
```

Alternatively, on a different OS or if you don't want to install system packages, you may use the
binaries included with the Arduino IDE. To do so, first find where Arduino preferences are located:

- Windows Store App: `%HOME%\Documents\ArduinoData\`
- Windows: `%APPDATA%\Arduino15\`
- macOS: `$HOME/Library/Arduino15/`
- Linux: `~/.arduino15/`

Now, append `packages/arduino/tools/avr-gcc/`, find the folder in that directory (e.g.
`7.3.0-atmel3.6.1-arduino7` at the time of writing), and then finally add `/bin`. For example, on
Linux, you may have `$HOME/.arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin`.

Once you have the above path, add it to your path. For example, on macOS or Linux, run
`export PATH="$HOME/.arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin:${PATH}"`.

## Initial installation

`async-avr` needs nightly rust, as of beginning 2021 a special release even see [Rahix/avr-hal#124](https://github.com/Rahix/avr-hal/issues/124)

create a file called `rust-toolchain` with the contents:

```toml
[toolchain]
channel = "nightly-2021-01-07"
components = ["rust-src"]
```

this file allows you to run all cargo commands with that nightly release automatically selected.

## Compiling and Running

We can compile by running

```bash
cargo build --examples --release
```

**Note:** If you didn't create `rust-toolchain` this might work:

```bash
cargo +nightly build --examples --release
```

Then, to upload it to a device, run:

```bash
avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/examples/serial.elf:e
```

Change the upload path (`target/avr-atmega328p/release/examples/serial.elf`) to meet what you want
to upload.

### If you only have the Arduino IDE installed

Enable "Show verbose output during: upload" in the Arduino IDE. Observe the build logs for an
`avrdude` command—it should look something like:

```bash
/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude -C/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:/tmp/arduino_build_721874/Blink.ino.hex:i
```

Copy that command, but delete everything after `-Uflash:w:`. Then, without spaces, add the path to
your binary. This will typically be `target/avr-atmega328p/release/project_name.elf`, or
`target/avr-atmega328p/release/examples/example_name.elf`. Finally, add `:e`. Your final command
will probably look something like:

```bash
/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude -C/path/to/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/example/serial.elf:e
```

> ### What about converting to hex first?
>
> Arduino typically converts the compiled binary to raw hex, and many AVR-Rust projects have
> [followed that pattern][avr-objcopy]. However, there's generally no need to do that, as `avrdude`
> has the ability to upload ELF binaries directly.

[avr-objcopy]:
  https://github.com/Rahix/avr-hal/blob/bfc5dfe67107a68b4a673e54532354af126cb3ba/mkhex.sh#L32
