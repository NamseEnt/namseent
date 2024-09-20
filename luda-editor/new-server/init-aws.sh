#!/bin/bash

awslocal s3 mb s3://visual-novel-database
awslocal s3 mb s3://visual-novel-asset

DATABASE_POLICY="{
  \"Version\": \"2012-10-17\",
  \"Statement\": [
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:GetObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-database/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:PutObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-database/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:ListObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-database\"
    }
  ]
}"
ASSET_POLICY="{
  \"Version\": \"2012-10-17\",
  \"Statement\": [
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:GetObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-asset/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:PutObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-asset/*\"
    },
    {
      \"Effect\": \"Allow\",
      \"Principal\": \"*\",
      \"Action\": \"s3:ListObject\",
      \"Resource\": \"arn:aws:s3:::visual-novel-asset\"
    }
  ]
}"

awslocal s3api put-bucket-policy \
    --bucket visual-novel-database \
    --policy "$DATABASE_POLICY"
awslocal s3api put-bucket-policy \
    --bucket visual-novel-asset \
    --policy "$ASSET_POLICY"

npm install -g aws-cdk-local aws-cdk

pushd /cdk
cdklocal bootstrap
echo "after bootstrap"
cdklocal deploy AudioTranscodingStack --require-approval=never
popd
