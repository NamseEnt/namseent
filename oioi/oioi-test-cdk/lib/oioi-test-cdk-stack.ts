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
    }
}
