docker-compose down

docker rm -v dynamodb-local
rm -rf ./dynamodblocal

docker rm -v s3-local
sudo rm -rf ./s3local