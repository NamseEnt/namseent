aws dynamodb create-table \
    --endpoint-url http://localhost:8000 \
    --table-name one-for-all \
    --attribute-definitions \
        AttributeName=p,AttributeType=S \
        AttributeName=s,AttributeType=S \
    --key-schema \
        AttributeName=p,KeyType=HASH \
        AttributeName=s,KeyType=RANGE \
    --billing-mode PAY_PER_REQUEST
