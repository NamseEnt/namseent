mkdir dynamodblocal
chmod 777 -R dynamodblocal

mkdir s3local
chmod 777 -R s3local

docker-compose up -d --force-recreate --remove-orphans
