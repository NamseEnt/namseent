export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123

echo "before create-table"
aws dynamodb create-table \
    --endpoint-url http://localhost:8000 \
    --table-name one-for-all \
    --attribute-definitions \
        AttributeName=__partition_key__,AttributeType=S \
        AttributeName=__sort_key__,AttributeType=S \
    --key-schema \
        AttributeName=__partition_key__,KeyType=HASH \
        AttributeName=__sort_key__,KeyType=RANGE \
    --billing-mode PAY_PER_REQUEST
echo "after create-table"

echo "before create-bucket"
aws s3api create-bucket \
    --endpoint-url http://localhost:9000 \
    --bucket one-for-all \
    --create-bucket-configuration LocationConstraint=ap-northeast-2
echo "after create-bucket"