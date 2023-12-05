import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class CicdStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const vpc = new cdk.aws_ec2.Vpc(this, "Vpc", {
            natGateways: 0,
            subnetConfiguration: [
                {
                    name: "public",
                    subnetType: cdk.aws_ec2.SubnetType.PUBLIC,
                },
            ],
        });

        const s3Bucket = new cdk.aws_s3.Bucket(this, "S3Bucket", {
            removalPolicy: cdk.RemovalPolicy.DESTROY,
        });

        const rustCicdSecurityGroup = new cdk.aws_ec2.SecurityGroup(
            this,
            "RustCicdSecurityGroup",
            {
                vpc,
                allowAllOutbound: true,
            },
        );
        rustCicdSecurityGroup.addIngressRule(
            cdk.aws_ec2.Peer.anyIpv4(),
            cdk.aws_ec2.Port.tcp(8080),
        );
    }
}
