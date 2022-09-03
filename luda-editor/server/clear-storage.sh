docker stop dynamodb-local
docker rm -v dynamodb-local
rm -rf ./dynamodblocal

docker stop s3-local
docker rm -v s3-local
sudo rm -rf ./s3local