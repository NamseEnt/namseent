import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as oioi from "@namseent/oioi";

export class OioiTestCdkStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const image =
            "public.ecr.aws/ecs-sample-image/amazon-ecs-sample:latest";

        const { vpc, autoScalingGroup } = new oioi.Oioi(this, "Oioi", {
            groupName: "test",
            image,
            portMappings: [
                {
                    containerPort: 80,
                    hostPort: 80,
                    protocol: "tcp",
                },
            ],
        });

        const albTargetGroup =
            new cdk.aws_elasticloadbalancingv2.ApplicationTargetGroup(
                this,
                "AlbTargetGroup",
                {
                    targets: [autoScalingGroup],
                    protocol:
                        cdk.aws_elasticloadbalancingv2.ApplicationProtocol.HTTP,
                    healthCheck: {
                        path: "/",
                        protocol: cdk.aws_elasticloadbalancingv2.Protocol.HTTP,
                    },
                    deregistrationDelay: cdk.Duration.seconds(10),
                    vpc,
                },
            );

        const alb = new cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer(
            this,
            "Alb",
            {
                vpc,
                internetFacing: true,
            },
        );

        alb.addListener("Listener", {
            port: 80,
            protocol: cdk.aws_elasticloadbalancingv2.ApplicationProtocol.HTTP,
            defaultTargetGroups: [albTargetGroup],
        });

        new cdk.CfnOutput(this, "LoadBalancerDnsName", {
            value: alb.loadBalancerDnsName,
        });

        // remove this stack after about 1 hour using lambda

        const lambda = new cdk.aws_lambda.Function(this, "Lambda", {
            // https://aws.amazon.com/ko/blogs/infrastructure-and-automation/scheduling-automatic-deletion-of-aws-cloudformation-stacks/
            code: cdk.aws_lambda.Code.fromInline(`
import boto3
import os
import json

stack_name = os.environ['STACK_NAME']

def delete_cfn(stack_name):
    try:
        cfn = boto3.resource('cloudformation')
        stack = cfn.Stack(stack_name)
        stack.delete()
        return "SUCCESS"
    except Exception as e:
        print(e)
        return "ERROR"

def handler(event, context):
    print("Received event:")
    print(json.dumps(event))
    return delete_cfn(stack_name)
            `),
            handler: "index.handler",
            runtime: cdk.aws_lambda.Runtime.PYTHON_3_12,
            architecture: cdk.aws_lambda.Architecture.ARM_64,
            timeout: cdk.Duration.minutes(1),
            initialPolicy: [
                new cdk.aws_iam.PolicyStatement({
                    actions: ["cloudformation:DeleteStack"],
                    resources: [
                        cdk.Stack.of(this).formatArn({
                            service: "cloudformation",
                            resource: "stack",
                            resourceName: this.stackName,
                        }),
                    ],
                }),
            ],
            environment: {
                STACK_NAME: this.stackName,
            },
        });

        const now = new Date();
        const minutes = now.getMinutes();

        const minus5Minutes = (60 + (minutes - 5)) % 60;

        new cdk.aws_events.Rule(this, "Rule", {
            schedule: cdk.aws_events.Schedule.cron({
                minute: minus5Minutes.toString(),
                hour: "*",
            }),
            targets: [new cdk.aws_events_targets.LambdaFunction(lambda)],
        });
    }
}
