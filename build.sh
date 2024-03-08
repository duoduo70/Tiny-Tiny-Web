echo ----------------------- Code Style Check -----------------------
cargo +stable clippy
echo ---------------------- Build Linux Version ---------------------
cargo +stable build --release
echo --------------------- Build Windows Version --------------------
cargo +stable build --release --target=x86_64-pc-windows-gnu

rm -rf ./build &&
mkdir -p ./build &&
mkdir -p ./build/config &&

cp ./target/release/ttweb ./build/ttweb &&
cp ./target/x86_64-pc-windows-gnu/release/ttweb.exe ./build/ttweb.exe &&
cp ./config/auto-image-hosting.gl ./build/config/auto-image-hosting.gl &&
cp ./config/markdown-auto-compile.gl ./build/config/markdown-auto-compile.gl &&
cp ./README.md ./build/README.md &&
cp ./docs/index.md ./build/doc.md &&
cp ./LICENSE ./build/LICENSE &&

echo -------------------- Build VSCode Extension ----------------------
cp ./build/ttweb ./vscode-ghost-lisp-extension/ &&
cp ./build/ttweb.exe ./vscode-ghost-lisp-extension/ &&
cd ./vscode-ghost-lisp-extension &&
npx vsce package &&
cp ./ghost-lisp-* ../build/ &&
cd ..

zip ./build/glisp-example.zip ./build/config/* &&
rm -rf ./build/config