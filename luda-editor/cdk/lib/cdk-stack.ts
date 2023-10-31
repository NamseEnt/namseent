import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as path from "path";

export class CdkStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const lambdaFunctionName = new cdk.CfnParameter(
            this,
            "lambdaFunctionName",
            {
                type: "String",
                description: "The name of the Lambda function",
            },
        );
        const githubClientId = new cdk.CfnParameter(this, "githubClientId", {
            type: "String",
            description: "Github Client Id",
        });
        const githubClientSecret = new cdk.CfnParameter(
            this,
            "githubClientSecret",
            {
                type: "String",
                description: "Github Client Secret",
            },
        );
        const serveStaticS3Bucket = new cdk.CfnParameter(
            this,
            "serveStaticS3Bucket",
            {
                type: "String",
                description: "The S3 Bucket of the static file to serve",
            },
        );
        const serveStaticS3KeyPrefix = new cdk.CfnParameter(
            this,
            "serveStaticS3KeyPrefix",
            {
                type: "String",
                description: "The S3 Key Prefix of the static file to serve",
            },
        );
        const s3BucketName = new cdk.CfnParameter(this, "s3BucketName", {
            type: "String",
            description: "The name of the S3 bucket",
        });
        const s3KeyPrefix = new cdk.CfnParameter(this, "s3KeyPrefix", {
            type: "String",
            description: "The prefix of the S3 key",
        });

        const dynamoDb = new cdk.aws_dynamodb.Table(this, "DynamoDb", {
            tableName: props?.stackName,
            billingMode: cdk.aws_dynamodb.BillingMode.PAY_PER_REQUEST,
            partitionKey: {
                name: "p",
                type: cdk.aws_dynamodb.AttributeType.STRING,
            },
            sortKey: {
                name: "s",
                type: cdk.aws_dynamodb.AttributeType.STRING,
            },
            removalPolicy:
                process.env.BRANCH_NAME === "master"
                    ? cdk.RemovalPolicy.RETAIN
                    : cdk.RemovalPolicy.DESTROY,
        });

        const lambda = new cdk.aws_lambda.Function(this, "Server", {
            runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2,
            handler: "bootstrap",
            code: cdk.aws_lambda.Code.fromAsset(
                path.join(
                    __dirname,
                    "../../server/server-bin/target/lambda/server-bin/bootstrap.zip",
                ),
            ),
            functionName: lambdaFunctionName.valueAsString,
            environment: {
                SERVE_STATIC_S3_BUCKET: serveStaticS3Bucket.valueAsString,
                SERVE_STATIC_S3_KEY_PREFIX:
                    serveStaticS3KeyPrefix.valueAsString,
                GITHUB_CLIENT_ID: githubClientId.valueAsString,
                GITHUB_CLIENT_SECRET: githubClientSecret.valueAsString,
                DYNAMODB_TABLE_NAME: dynamoDb.tableName,
                S3_BUCKET_NAME: s3BucketName.valueAsString,
                S3_KEY_PREFIX: s3KeyPrefix.valueAsString,
            },
            initialPolicy: [
                new cdk.aws_iam.PolicyStatement({
                    resources: [
                        `arn:aws:s3:::${s3BucketName.valueAsString}/${s3KeyPrefix.valueAsString}*`,
                    ],
                    actions: [
                        "s3:GetObject",
                        "s3:PutObject",
                        "s3:PutObjectAcl",
                    ],
                }),
                new cdk.aws_iam.PolicyStatement({
                    resources: [dynamoDb.tableArn],
                    actions: [
                        "dynamodb:PutItem",
                        "dynamodb:GetItem",
                        "dynamodb:DeleteItem",
                        "dynamodb:Query",
                        "dynamodb:ConditionCheck",
                    ],
                }),
            ],
            timeout: cdk.Duration.seconds(30),
        });

        const fnUrl = lambda.addFunctionUrl({
            authType: cdk.aws_lambda.FunctionUrlAuthType.NONE,
        });

        new cdk.CfnOutput(this, "FunctionUrl", {
            value: fnUrl.url,
        });
    }
}
