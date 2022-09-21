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
