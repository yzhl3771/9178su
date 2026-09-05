alias bk := build_n9178sud
alias bm := build_manager

build_n9178sud:
    cross build --target aarch64-linux-android --release

build_manager: build_n9178sud
    cp target/aarch64-linux-android/release/n9178sud manager/app/src/main/jniLibs/arm64-v8a/libn9178sud.so
    cd manager && ./gradlew aDebug

clippy:
    cargo fmt
    cross clippy --target aarch64-linux-android --release
