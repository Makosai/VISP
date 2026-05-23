set windows-shell := ["cmd.exe", "/c"]

default:
    @just --list

setup:
    @echo "Installing flutter_rust_bridge_codegen..."
    cargo install flutter_rust_bridge_codegen
    @echo "Activating Melos globally..."
    dart pub global activate melos
    just bootstrap

bootstrap:
    dart pub get
    melos run workspace-bootstrap

codegen:
    melos run codegen

build-windows:
    just codegen
    cd packages/visp_core && cargo build
    cd apps/visp_desktop && flutter build windows --debug

run-windows:
    just build-windows
    start apps/visp_desktop/build/windows/x64/runner/Debug/visp_desktop.exe

run target="windows":
    just codegen
    cd apps/visp_desktop && flutter run -d {{target}}

test-core:
    cd packages/visp_core && cargo test

run-flutter target="windows":
    cd apps/visp_desktop && flutter run -d {{target}}

check-dart:
    melos exec -- flutter analyze

clean:
    cd apps/visp_desktop && flutter clean
    cd packages/visp_core && cargo clean
    rm -rf target
