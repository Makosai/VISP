default:
    @just --list

setup:
    @echo "Installing flutter_rust_bridge_codegen..."
    cargo install flutter_rust_bridge_codegen --version ^2.0.0
    @echo "Activating Melos globally..."
    dart pub global activate melos
    just bootstrap

bootstrap:
    dart pub get
    melos run bootstrap

codegen:
    melos run codegen

run target="windows":
    just codegen
    cd apps/visp_desktop && flutter run -d {{target}}
