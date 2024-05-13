import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import path = require("path");

export class VisualNovelStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const s3Bucket = new cdk.aws_s3.Bucket(this, "S3Bucket", {
            removalPolicy: cdk.RemovalPolicy.RETAIN,
        });

        const sslCertRenewalLambda = new cdk.aws_lambda.Function(
            this,
            "SSLCertRenewalLambda",
            {
                code: cdk.aws_lambda.Code.fromAssetImage(
                    path.join(__dirname, "sslCertRenewal"),
                ),
                handler: "function.handler",
                runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2023,
                environment: {
                    BUCKET_NAME: s3Bucket.bucketName,
                },
                role: new cdk.aws_iam.Role(this, "sslCertRenewalLambdaRole", {
                    assumedBy: new cdk.aws_iam.ServicePrincipal(
                        "lambda.amazonaws.com",
                    ),
                    inlinePolicies: {
                        InstancePolicy: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    effect: cdk.aws_iam.Effect.ALLOW,
                                    actions: [
                                        "route53:ListHostedZones",
                                        "route53:GetChange",
                                    ],
                                    resources: ["*"],
                                }),
                                new cdk.aws_iam.PolicyStatement({
                                    effect: cdk.aws_iam.Effect.ALLOW,
                                    actions: [
                                        "route53:ChangeResourceRecordSets",
                                    ],
                                    resources: [
                                        "arn:aws:route53:::hostedzone/Z03861008D2C0NOIITVX",
                                    ],
                                }),
                                new cdk.aws_iam.PolicyStatement({
                                    effect: cdk.aws_iam.Effect.ALLOW,
                                    actions: [
                                        "s3:DeleteObject",
                                        "s3:GetBucketLocation",
                                        "s3:GetObject",
                                        "s3:ListBucket",
                                        "s3:PutObject",
                                    ],
                                    resources: [s3Bucket.bucketArn],
                                }),
                            ],
                        }),
                    },
                }),
            },
        );

        const sslCertRenewalRule = new cdk.aws_events.Rule(
            this,
            "SSLCertRenewalRule",
            {
                schedule: cdk.aws_events.Schedule.expression("rate(30 days)"),
            },
        );

        sslCertRenewalRule.addTarget(
            new cdk.aws_events_targets.LambdaFunction(sslCertRenewalLambda),
        );

        const vpc = new cdk.aws_ec2.Vpc(this, "VPC", {
            ipProtocol: cdk.aws_ec2.IpProtocol.DUAL_STACK,
            natGateways: 0,
            subnetConfiguration: [
                {
                    name: "public",
                    subnetType: cdk.aws_ec2.SubnetType.PUBLIC,
                    mapPublicIpOnLaunch: false,
                },
            ],
        });

        const fleetRole = new cdk.aws_iam.Role(this, "FleetRole", {
            assumedBy: new cdk.aws_iam.ServicePrincipal("ec2.amazonaws.com"),
            inlinePolicies: {
                FleetPolicy: new cdk.aws_iam.PolicyDocument({
                    statements: [
                        new cdk.aws_iam.PolicyStatement({
                            effect: cdk.aws_iam.Effect.ALLOW,
                            actions: [
                                "ec2:RunInstances",
                                "ec2:CreateTags",
                                "ec2:RequestSpotFleet",
                                "ec2:ModifySpotFleetRequest",
                                "ec2:CancelSpotFleetRequests",
                                "ec2:DescribeSpotFleetRequests",
                                "ec2:DescribeSpotFleetInstances",
                                "ec2:DescribeSpotFleetRequestHistory",
                            ],
                            resources: ["*"],
                        }),
                        new cdk.aws_iam.PolicyStatement({
                            effect: cdk.aws_iam.Effect.ALLOW,
                            actions: ["iam:PassRole"],
                            resources: [
                                "arn:aws:iam::*:role/aws-ec2-spot-fleet-tagging-role",
                            ],
                        }),
                        new cdk.aws_iam.PolicyStatement({
                            effect: cdk.aws_iam.Effect.ALLOW,
                            actions: [
                                "iam:CreateServiceLinkedRole",
                                "iam:ListRoles",
                                "iam:ListInstanceProfiles",
                            ],
                            resources: ["*"],
                        }),
                    ],
                }),
            },
        });

        const instanceRole = new cdk.aws_iam.Role(this, "InstanceRole", {
            assumedBy: new cdk.aws_iam.ServicePrincipal("ec2.amazonaws.com"),
            inlinePolicies: {
                InstancePolicy: new cdk.aws_iam.PolicyDocument({
                    statements: [
                        new cdk.aws_iam.PolicyStatement({
                            effect: cdk.aws_iam.Effect.ALLOW,
                            actions: [
                                "s3:DeleteObject",
                                "s3:GetBucketLocation",
                                "s3:GetObject",
                                "s3:ListBucket",
                                "s3:PutObject",
                            ],
                            resources: [s3Bucket.bucketArn],
                        }),
                    ],
                }),
            },
        });

        const instanceProfile = new cdk.aws_iam.CfnInstanceProfile(
            this,
            "InstanceProfile",
            {
                roles: [instanceRole.roleName],
            },
        );

        const securityGroup = new cdk.aws_ec2.SecurityGroup(
            this,
            "SecurityGroup",
            {
                vpc,
                allowAllIpv6Outbound: true,
                allowAllOutbound: true,
            },
        );

        securityGroup.addIngressRule(
            cdk.aws_ec2.Peer.ipv4("13.209.1.56/29"),
            cdk.aws_ec2.Port.tcp(22),
            "Allow SSH access for ec2 connect",
        );

        securityGroup.addIngressRule(
            cdk.aws_ec2.Peer.anyIpv4(),
            cdk.aws_ec2.Port.tcpRange(8000, 8999),
        );
        securityGroup.addIngressRule(
            cdk.aws_ec2.Peer.anyIpv6(),
            cdk.aws_ec2.Port.tcpRange(8000, 8999),
        );

        [443].forEach((port) => {
            securityGroup.addIngressRule(
                cdk.aws_ec2.Peer.anyIpv4(),
                cdk.aws_ec2.Port.tcp(port),
            );
            securityGroup.addIngressRule(
                cdk.aws_ec2.Peer.anyIpv6(),
                cdk.aws_ec2.Port.tcp(port),
            );
        });

        const cfnSpotFleet = new cdk.aws_ec2.CfnSpotFleet(this, "SpotFleet", {
            spotFleetRequestConfigData: {
                iamFleetRole: fleetRole.roleArn,
                targetCapacity: 1,

                // the properties below are optional
                allocationStrategy: "priceCapacityOptimized",
                instanceInterruptionBehavior: "stop",
                launchSpecifications: [
                    {
                        imageId: cdk.aws_ec2.MachineImage.latestAmazonLinux2023(
                            {
                                cpuType: cdk.aws_ec2.AmazonLinuxCpuType.ARM_64,
                            },
                        ).getImage(this).imageId,

                        // the properties below are optional
                        blockDeviceMappings: [
                            {
                                deviceName: "/dev/sdh",
                                ebs: {
                                    deleteOnTermination: true,
                                    iops: 3000,
                                    volumeSize: 30,
                                    volumeType: "gp3",
                                },
                            },
                        ],
                        iamInstanceProfile: {
                            arn: instanceProfile.attrArn,
                        },
                        instanceType: "t4g.nano",
                        securityGroups: [
                            {
                                groupId: securityGroup.securityGroupId,
                            },
                        ],
                        subnetId: `${vpc.publicSubnets[0].subnetId},${vpc.publicSubnets[1].subnetId},${vpc.publicSubnets[2].subnetId}`,
                        userData: `#!/bin/bash
dnf install -y \\
    cronie \\
    cronie-anacron \\
    git \\
    curl

systemctl enable crond
systemctl start crond

# create 4G swap memory
fallocate -l 4G /swapfile
chmod 600 /swapfile
mkswap /swapfile
swapon /swapfile

# set environment variables
echo export BUCKET_NAME=${s3Bucket.bucketName} >> /etc/profile

# sync certs from s3
aws s3 sync s3://$BUCKET_NAME/certs /etc/letsencrypt/live/visual-novel.namseent.com
(crontab -l 2>/dev/null; echo "0 0 * * * aws s3 sync s3://$BUCKET_NAME/certs /etc/letsencrypt/live/visual-novel.namseent.com") | crontab -

# git setup
mkdir -p /namseent
cd /namseent
git clone --filter=blob:none --no-checkout https://github.com/namseent/namseent.git /namseent
git sparse-checkout set /luda-editor/new-server/
git checkout master

# install rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"

# run manager
echo export RUST_BACKTRACE=1 >> /etc/profile
echo export IS_ON_AWS=true >> /etc/profile

cd /namseent/luda-editor/new-server/manager
nohup cargo run --release > /dev/null 2>&1 &
echo export MANAGER_PID=$! >> /etc/profile

# shutdown instance when the manager process is not running
(crontab -l 2>/dev/null; echo "1 0 * * * if ! ps -p $MANAGER_PID > /dev/null; then shutdown -h now; fi") | crontab -
`,
                    },
                ],
                replaceUnhealthyInstances: true,
                targetCapacityUnitType: "units",
                terminateInstancesWithExpiration: true,
                type: "maintain",
            },
        });
    }
}
