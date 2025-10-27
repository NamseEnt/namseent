rm combined.bc combined.ll

llvm-link ./target/wasm32-wasip1-threads/debug/deps/*.ll -o combined.bc
llvm-dis combined.bc combined.ll