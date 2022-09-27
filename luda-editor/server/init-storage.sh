export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123

echo "before create-table"
./create_table.sh
echo "after create-table"

echo "before create-bucket"
aws s3api create-bucket \
    --endpoint-url http://localhost:9000 \
    --bucket one-for-all \
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
      \"Resource\": \"arn:aws:s3:::one-for-all/*\"
    }
  ]
}"

aws s3api put-bucket-policy \
    --endpoint-url http://localhost:9000 \
    --bucket one-for-all \
    --policy "$POLICY"
echo "after bucket policy public image setting"