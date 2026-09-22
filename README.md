# ExoMar7-STM32
This is the source code for the STM32H562 MCU in the ExoMar7 robot.

# Prerequisites

- [Zephyr project](https://docs.zephyrproject.org/latest/develop/getting_started/index.html) (Presumed install directory: ~/zephyrproject)
- [Rust tools](https://rustup.rs/)
- [imgtool](https://pypi.org/project/imgtool/)
- [just](https://github.com/casey/just)
- [LLVM 18](https://github.com/llvm/llvm-project/releases?page=6#release-llvmorg-18.1.8) (Presumed install directory: /opt/llvm-18/)

If any of those install directories is different, they can be changed in the justfile

> [!TIP]
> In order to get autocompletion working on Zephyr items it is required to build the project at least once.

# Building
Run the following command:
```bash
just build
```
Or, if flashing is also required:
```bash
just flash
```
If you instead want to do a pristine build, run:
```bash
just pristine
```
