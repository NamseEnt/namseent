export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123

docker compose up -d --remove-orphans

echo "before create-bucket"
aws s3api create-bucket \
  --endpoint-url http://localhost:9000 \
  --bucket visual-novel \
  --create-bucket-configuration LocationConstraint=ap-northeast-2
echo "after create-bucket"

echo "before bucket policy public image setting"
POLICY="{
  \"Version\": \"2012-10-17\",
  \"Statement\": [
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:GetObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:PutObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:ListObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel\"
    }
  ]
}"

aws s3api put-bucket-policy \
  --endpoint-url http://localhost:9000 \
  --bucket visual-novel \
  --policy "$POLICY"
echo "after bucket policy public image setting"
