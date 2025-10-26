RUSTFLAGS="-Zalways-encode-mir" cargo +nightly build \
  --target wasm32-wasip1-threads \
  -Z build-std=std,core

cd ..

# Sysroot 디렉토리 구조 생성
mkdir -p custom-sysroot/lib/rustlib/wasm32-wasip1-threads/lib

# 빌드된 라이브러리 파일 복사
cp sysroot-builder/target/wasm32-wasip1-threads/debug/deps/*.rlib \
   custom-sysroot/lib/rustlib/wasm32-wasip1-threads/lib/

rm -rf ../custom-sysroot
mv custom-sysroot ../