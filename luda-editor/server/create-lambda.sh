aws lambda create-function --function-name rustTest \
  --handler bootstrap \
  --zip-file fileb://./target/lambda/release/server.zip \
  --runtime provided.al2 \
  --role arn:aws:iam::304616328860:role/luda-editor-lambda-role \
  --environment Variables={RUST_BACKTRACE=1} \
  --tracing-config Mode=Active
