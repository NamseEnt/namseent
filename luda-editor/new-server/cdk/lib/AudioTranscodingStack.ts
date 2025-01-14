import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import path = require("path");

export class AudioTranscodingStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const isLocalstack = !!process.env.IS_LOCALSTACK;
        const localstackAssetBucketName = "visual-novel-asset";

        const assetBucket = new cdk.aws_s3.Bucket(this, "AssetBucket", {
            bucketName: isLocalstack ? localstackAssetBucketName : undefined,
            blockPublicAccess: cdk.aws_s3.BlockPublicAccess.BLOCK_ACLS,
            publicReadAccess: true,
            cors: [
                ...(isLocalstack
                    ? [
                          {
                              allowedMethods: [
                                  cdk.aws_s3.HttpMethods.DELETE,
                                  cdk.aws_s3.HttpMethods.GET,
                                  cdk.aws_s3.HttpMethods.POST,
                                  cdk.aws_s3.HttpMethods.PUT,
                              ],
                              allowedOrigins: ["*"],
                          },
                      ]
                    : []),
            ],
        });

        assetBucket.addToResourcePolicy(
            new cdk.aws_iam.PolicyStatement({
                effect: cdk.aws_iam.Effect.ALLOW,
                actions: ["s3:GetObject"],
                resources: [`${assetBucket.bucketArn}/*`],
                principals: [new cdk.aws_iam.AnyPrincipal()],
            }),
        );

        if (isLocalstack) {
            assetBucket.addToResourcePolicy(
                new cdk.aws_iam.PolicyStatement({
                    effect: cdk.aws_iam.Effect.ALLOW,
                    actions: ["s3:PutObject"],
                    resources: [`${assetBucket.bucketArn}/*`],
                    principals: [new cdk.aws_iam.AnyPrincipal()],
                }),
            );
            assetBucket.addToResourcePolicy(
                new cdk.aws_iam.PolicyStatement({
                    effect: cdk.aws_iam.Effect.ALLOW,
                    actions: ["s3:ListObject"],
                    resources: [assetBucket.bucketArn],
                    principals: [new cdk.aws_iam.AnyPrincipal()],
                }),
            );
        }

        const audioTranscodingLambda = new cdk.aws_lambda.Function(
            this,
            "audioTranscodingLambda",
            {
                code: cdk.aws_lambda.Code.fromAsset(
                    path.join(
                        __dirname,
                        "audio-transcoding-lambda-code/archive.zip",
                    ),
                ),
                handler: "function.handler",
                runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2023,
                environment: {
                    BUCKET_NAME: isLocalstack
                        ? localstackAssetBucketName
                        : assetBucket.bucketName,
                    RUST_BACKTRACE: "1",
                },
                architecture: cdk.aws_lambda.Architecture.X86_64,
                role: new cdk.aws_iam.Role(this, "audioTranscodingLambdaRole", {
                    assumedBy: new cdk.aws_iam.ServicePrincipal(
                        "lambda.amazonaws.com",
                    ),
                    inlinePolicies: {
                        InstancePolicy: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    effect: cdk.aws_iam.Effect.ALLOW,
                                    actions: [
                                        "s3:DeleteObject",
                                        "s3:GetObject",
                                        "s3:ListBucket",
                                        "s3:PutObject",
                                    ],
                                    resources: [
                                        isLocalstack
                                            ? `arn:aws:s3:::${localstackAssetBucketName}`
                                            : assetBucket.bucketArn,
                                    ],
                                }),
                            ],
                        }),
                    },
                }),
            },
        );

        if (isLocalstack) {
            // cannot use add_event_notification in localstack
            // https://github.com/localstack/localstack/issues/9352#issuecomment-1862125662
            // and this will make a circular dependency, so directly use bucket name where bucketArn or name is needed
            const cfnBucket = assetBucket.node
                .defaultChild as cdk.aws_s3.CfnBucket;
            cfnBucket.notificationConfiguration = {
                lambdaConfigurations: [
                    {
                        creationStack: [],
                        event: "s3:ObjectCreated:*",
                        function: audioTranscodingLambda.functionArn,
                        filter: {
                            s3Key: {
                                rules: [
                                    {
                                        name: "prefix",
                                        value: "audio/before-transcode/",
                                    },
                                ],
                            },
                        },
                    },
                ],
            };
        } else {
            assetBucket.addEventNotification(
                cdk.aws_s3.EventType.OBJECT_CREATED,
                new cdk.aws_s3_notifications.LambdaDestination(
                    audioTranscodingLambda,
                ),
                {
                    prefix: "audio/before-transcode/",
                },
            );
        }

        new cdk.CfnOutput(this, "AssetBucketName", {
            value: assetBucket.bucketName,
        });
    }
}
