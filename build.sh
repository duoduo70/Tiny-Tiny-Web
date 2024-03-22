#!/bin/sh

# 这个脚本用来快速构建本项目
# Usage:
#   build.sh [args]                 快速构建本地版本
#   build.sh all                    以常规方式构建所有官方支持的版本
#   build.sh local-sm               构建最小二进制大小的本地版本
#   build.sh nogl <target> [args]   手动编译不包含 Ghost Lisp 的版本
#   build.sh vsc                    构建 VSCode 插件
#   build.sh publish                构建在 Github（或其它） 发布的版本

echo "----------------------- Code Style Check -----------------------"
cargo clippy &&

rm -rf ./build &&
mkdir -p ./build &&
mkdir -p ./build/config &&

if [ $1 == "all" ]; then
    echo "---------------------- Build Linux Version ---------------------"
    cargo +stable build --release --target=x86_64-unknown-linux-gnu
    echo "--------------------- Build Windows Version --------------------"
    cargo +stable build --release --target=x86_64-pc-windows-gnu
    cp ./target/x86_64-unknown-linux-gnu/release/ttweb ./build/ttweb &&
    cp ./target/x86_64-pc-windows-gnu/release/ttweb.exe ./build/ttweb.exe
elif [ $1 == "local-sm" ]; then
    echo "-------------- Build Linux Version (Smallest) ------------------"
    cargo +nightly build --release --target=x86_64-unknown-linux-gnu -Zbuild-std
    cp ./target/x86_64-unknown-linux-gnu/release/ttweb ./build/ttweb
elif [ $1 == "nogl"  ]; then
    echo "--------------- Build Linux Version (nogl) ---------------------"
    cargo +stable build --release --target=$2 --features no-glisp $3 $4 $5 $6 $7
elif [ $1 == "vsc" ]; then
    echo "---------------------- Build Linux Version ---------------------"
    cargo +stable build --release --target=x86_64-unknown-linux-gnu
    echo "--------------------- Build Windows Version --------------------"
    cargo +stable build --release --target=x86_64-pc-windows-gnu
    cp ./target/x86_64-unknown-linux-gnu/release/ttweb ./build/ttweb &&
    cp ./target/x86_64-pc-windows-gnu/release/ttweb.exe ./build/ttweb.exe
    echo "-------------------- Build VSCode Extension ----------------------"
    cp ./build/ttweb ./vscode-ghost-lisp-extension/ &&
    cp ./build/ttweb.exe ./vscode-ghost-lisp-extension/ &&
    cd ./vscode-ghost-lisp-extension &&
    npx vsce package &&
    cp *.vsix ../build/ &&
    rm *.vsix
    cd ..
elif [ $1 == "publish" ]; then
    echo "-------------- Build Linux Version (Smallest) ------------------"
    cargo +nightly build --release --target=x86_64-unknown-linux-gnu -Zbuild-std
    echo "--------------------- Build Windows Version --------------------"
    cargo +stable build --release --target=x86_64-pc-windows-gnu
    cp ./target/x86_64-unknown-linux-gnu/release/ttweb ./build/ttweb &&
    cp ./target/x86_64-pc-windows-gnu/release/ttweb.exe ./build/ttweb.exe

    echo "-------------------- Build VSCode Extension ----------------------"
    cp ./build/ttweb ./vscode-ghost-lisp-extension/ &&
    cp ./build/ttweb.exe ./vscode-ghost-lisp-extension/ &&
    cd ./vscode-ghost-lisp-extension &&
    npx vsce package &&
    cp *.vsix ../build/ &&
    rm *.vsix
    cd ..
else
    echo "---------------------- Build Linux Version ---------------------"
    cargo +stable build --release $1
    cp ./target/x86_64-unknown-linux-gnu/release/ttweb ./build/ttweb
fi

cp ./config/auto-image-hosting.gl ./build/config/auto-image-hosting.gl &&
cp ./config/markdown-auto-compile.gl ./build/config/markdown-auto-compile.gl &&
cp ./README.md ./build/README.md &&
cp ./docs/index.md ./build/doc.md &&
cp ./LICENSE ./build/LICENSE &&

zip ./build/glisp-example.zip ./build/config/* &&
rm -rf ./build/config