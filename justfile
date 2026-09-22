set shell := ["bash", "-euo", "pipefail", "-c"]

# --- Configuration ----------------------------------------------------------
board        := "weact_stm32h562_core"
build_dir    := "build"
project_root := justfile_directory()

home         := env("HOME")
zephyr_venv  := home + "/zephyrproject/.venv"
llvm_lib     := "/opt/llvm-18/lib"
compat_hdr   := project_root + "/picolibc_includes/bindgen_gcc_compat.h"

# --- Environment (exported to every recipe) ---------------------------------
export PATH := zephyr_venv + "/bin:" + env("PATH")
export LIBCLANG_PATH := llvm_lib
export BINDGEN_EXTRA_CLANG_ARGS := "-include " + compat_hdr

# List recipes
default:
    @just --list

build:
    west build -b {{board}} --build-dir {{build_dir}} --sysbuild -- -DSB_CONFIG_BOOT_SIGNATURE_KEY_FILE=\"{{project_root}}/keys/ecdsa-p256.pem\"

pristine:
    west build -p always -b {{board}} --build-dir {{build_dir}} --sysbuild -- -DSB_CONFIG_BOOT_SIGNATURE_KEY_FILE=\"{{project_root}}/keys/ecdsa-p256.pem\"

flash: build
    west flash

menuconfig:
    west build --build-dir {{build_dir}} -t menuconfig

clean:
    rm -rf {{build_dir}}

check:
    cargo check
