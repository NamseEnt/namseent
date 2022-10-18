#!/bin/bash

if [ $# -ne 2 ]; then
 echo "Usage: $0 [partition_key] [sort_key]"
 exit -1
fi

key=$(cat <<END
{"__partition_key__": {"S": "$1"}, "__sort_key__": {"S": "$2"}}
END
)

aws dynamodb delete-item \
    --endpoint-url http://localhost:8000 \
    --table-name one-for-all \
    --key "$key"
