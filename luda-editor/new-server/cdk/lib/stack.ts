import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class VisualNovelStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const s3Bucket = new cdk.aws_s3.Bucket(this, "S3Bucket", {
            removalPolicy: cdk.RemovalPolicy.RETAIN,
        });

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

        const elasticIp = new cdk.aws_ec2.CfnEIP(this, "ElasticIp", {
            domain: "vpc",
        });
        elasticIp.addDependency(vpc.node.defaultChild as cdk.CfnResource);

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
        instanceRole.addManagedPolicy(
            cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName(
                "CloudWatchAgentServerPolicy",
            ),
        );

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
            cdk.aws_ec2.Peer.prefixList("pl-00ec8fd779e5b4175"),
            cdk.aws_ec2.Port.tcp(22),
            "Allow SSH access for ec2 connect",
        );
        securityGroup.addIngressRule(
            cdk.aws_ec2.Peer.prefixList("pl-075e2b43f16f625b8"),
            cdk.aws_ec2.Port.tcp(22),
            "Allow SSH access for ec2 connect via ipv6",
        );

        // https://www.cloudflare.com/ips-v4
        const cloudflareIpv4Range = [
            "173.245.48.0/20",
            "103.21.244.0/22",
            "103.22.200.0/22",
            "103.31.4.0/22",
            "141.101.64.0/18",
            "108.162.192.0/18",
            "190.93.240.0/20",
            "188.114.96.0/20",
            "197.234.240.0/22",
            "198.41.128.0/17",
            "162.158.0.0/15",
            "104.16.0.0/13",
            "104.24.0.0/14",
            "172.64.0.0/13",
            "131.0.72.0/22",
        ];
        for (const ipv4 of cloudflareIpv4Range) {
            securityGroup.addIngressRule(
                cdk.aws_ec2.Peer.ipv4(ipv4),
                cdk.aws_ec2.Port.tcp(8080),
                "Allow Cloudfront",
            );
        }

        // https://www.cloudflare.com/ips-v6
        const cloudflareIpv6Range = [
            "2400:cb00::/32",
            "2606:4700::/32",
            "2803:f800::/32",
            "2405:b500::/32",
            "2405:8100::/32",
            "2a06:98c0::/29",
            "2c0f:f248::/32",
        ];

        for (const ipv6 of cloudflareIpv6Range) {
            securityGroup.addIngressRule(
                cdk.aws_ec2.Peer.ipv6(ipv6),
                cdk.aws_ec2.Port.tcp(8080),
                "Allow Cloudfront",
            );
        }

        const userDataScript = `#!/bin/bash
set -e

echo export BUCKET_NAME=${s3Bucket.bucketName} >> /etc/profile
echo export ELASTIC_IP_ALLOCATION_ID=${
            elasticIp.attrAllocationId
        } >> /etc/profile

yum install amazon-cloudwatch-agent
/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
    -a fetch-config \
    -m ec2 \
    -s

dnf install -y \\
    cronie \\
    cronie-anacron \\
    curl

systemctl enable crond
systemctl start crond

BINARY_PULL_SCRIPT=$(echo "${atob(binaryPullScript)}" | base64 -w 0)
(crontab -l 2>/dev/null; echo "*/1 * * * * echo $BINARY_PULL_SCRIPT | base64 -d | bash") | crontab -

EIP_ASSOCIATE_SCRIPT=$(echo "${atob(elasticIpAssociateScript)}" | base64 -w 0)
(crontab -l 2>/dev/null; echo "*/1 * * * * echo $EIP_ASSOCIATE_SCRIPT | base64 -d | bash") | crontab -
`;

        const autoScalingGroup = new cdk.aws_autoscaling.AutoScalingGroup(
            this,
            "AutoScalingGroup",
            {
                vpc,
                allowAllOutbound: true,
                // Free until 2024-12-31 - https://aws.amazon.com/ec2/faqs/#t4g-instances
                instanceType: cdk.aws_ec2.InstanceType.of(
                    cdk.aws_ec2.InstanceClass.T4G,
                    cdk.aws_ec2.InstanceSize.SMALL,
                ),
                machineImage: cdk.aws_ec2.MachineImage.latestAmazonLinux2023({
                    cpuType: cdk.aws_ec2.AmazonLinuxCpuType.ARM_64,
                }),
                minCapacity: 1,
                maxCapacity: 1,
                role: instanceRole,
                blockDevices: [
                    {
                        deviceName: "/dev/sdh",
                        volume: cdk.aws_autoscaling.BlockDeviceVolume.ebs(12, {
                            volumeType:
                                cdk.aws_autoscaling.EbsDeviceVolumeType.GP3,
                        }),
                    },
                ],
                securityGroup,
                vpcSubnets: { subnetType: cdk.aws_ec2.SubnetType.PUBLIC },
                userData: cdk.aws_ec2.UserData.custom(userDataScript),
            },
        );
    }
}

const elasticIpAssociateScript = `#!/bin/bash
set -e

INSTANCE_ID=$(curl -s http://169.254.169.254/latest/meta-data/instance-id)
aws ec2 associate-address --instance-id $INSTANCE_ID --allocation-id $ELASTIC_IP_ALLOCATION_ID --allow-reassociation

`;

const binaryPullScript = `#!/bin/bash
set -e

LAST_MODIFIED="Thu, 01 Jan 1970 00:00:00 GMT"
if [ -f /tmp/last_modified ]; then
    LAST_MODIFIED=$(cat /tmp/last_modified)
fi

RESPONSE=$(curl -s -w "%{http_code}" \
    -o /tmp/binary \
    -D /tmp/headers \
    --header "If-Modified-Since: $LAST_MODIFIED" \
    https://$BUCKET_NAME.s3.amazonaws.com/binary)

if [ "$RESPONSE" -eq 200 ]; then
    # Update the last modified date from the response headers
    LAST_MODIFIED=$(grep -i "Last-Modified" /tmp/headers | awk '{print substr($0, index($0,$2))}')
    echo $LAST_MODIFIED > /tmp/last_modified

    # Stop the running binary
    pkill -f ~/binary

    # Replace the old binary with the new one
    mv /tmp/binary ~/binary
    chmod +x ~/binary

    # Start the new binary
    nohup ~/binary
fi`;
