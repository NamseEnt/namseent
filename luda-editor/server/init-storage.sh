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