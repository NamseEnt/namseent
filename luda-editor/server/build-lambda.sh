# NOTE: install cargo lambda first
# pip3 install cargo-lambda

cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip
