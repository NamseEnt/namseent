import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { CloudfrontVpcOriginRemover } from "./CloudfrontVpcOrigin";

export class VisualNovelStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const s3Bucket = new cdk.aws_s3.Bucket(this, "S3Bucket", {
            removalPolicy: cdk.RemovalPolicy.RETAIN,
        });

        const vpc = new cdk.aws_ec2.Vpc(this, "VPC", {
            ipProtocol: cdk.aws_ec2.IpProtocol.DUAL_STACK,
            natGateways: 0,
            // apne2-az1 not support vpc origin
            // https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-vpc-origins.html#vpc-origins-supported-regions
            availabilityZones: ["b", "c", "d"].map(
                (az) => `ap-northeast-2${az}`,
            ),
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

        const frontendDomain = "visual-novel.namseent.com";

        const responseHeadersPolicy =
            new cdk.aws_cloudfront.ResponseHeadersPolicy(
                this,
                "ResponseHeadersPolicy",
                {
                    corsBehavior: {
                        accessControlAllowOrigins: [frontendDomain],
                        accessControlAllowMethods: ["GET", "HEAD", "OPTIONS"],
                        accessControlAllowCredentials: true,
                        accessControlAllowHeaders: ["*"],
                        originOverride: false,
                    },
                },
            );

        const cloudfrontDistribution = new cdk.aws_cloudfront.Distribution(
            this,
            "CloudfrontDistribution",
            {
                defaultBehavior: {
                    origin: new cdk.aws_cloudfront_origins.HttpOrigin(
                        "namseent.com",
                    ),
                    allowedMethods:
                        cdk.aws_cloudfront.AllowedMethods
                            .ALLOW_GET_HEAD_OPTIONS,
                    cachePolicy:
                        cdk.aws_cloudfront.CachePolicy.CACHING_DISABLED,
                    originRequestPolicy:
                        cdk.aws_cloudfront.OriginRequestPolicy.ALL_VIEWER,
                    responseHeadersPolicy,
                    viewerProtocolPolicy:
                        cdk.aws_cloudfront.ViewerProtocolPolicy
                            .REDIRECT_TO_HTTPS,
                },
            },
        );

        const domainName = "vn-api.namseent.com";

        new cdk.aws_route53.ARecord(this, "Route53Record", {
            zone: cdk.aws_route53.HostedZone.fromLookup(this, "HostedZone", {
                domainName: "namseent.com",
            }),
            target: cdk.aws_route53.RecordTarget.fromAlias(
                new cdk.aws_route53_targets.CloudFrontTarget(
                    cloudfrontDistribution,
                ),
            ),
            recordName: domainName,
        });

        // make stack unique name
        const vpcOriginName = this.stackName;

        const userDataScript = `#!/bin/bash
set -e

echo export BUCKET_NAME=${s3Bucket.bucketName} >> /etc/profile
echo export VPC_ORIGIN_NAME=${vpcOriginName} >> /etc/profile
echo export CLOUDFRONT_DISTRIBUTION_ID=${
            cloudfrontDistribution.distributionId
        } >> /etc/profile
echo export DOMAIN_NAME=${domainName} >> /etc/profile

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

VPC_ORIGIN_UPDATE_SCRIPT=$(echo "${atob(vpcOriginUpdateScript)}" | base64 -w 0)
(crontab -l 2>/dev/null; echo "*/1 * * * * echo $VPC_ORIGIN_UPDATE_SCRIPT | base64 -d | bash") | crontab -
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

        new CloudfrontVpcOriginRemover(this, "CloudfrontVpcOriginRemover", {
            vpcOriginName,
        });
    }
}

const vpcOriginUpdateScript = `#!/bin/bash
set -e

INSTANCE_ID=$(curl -s http://169.254.169.254/latest/meta-data/instance-id)
INSTANCE_ARN=$(aws ec2 describe-instances --instance-ids $INSTANCE_ID --query "Reservations[0].Instances[0].IamInstanceProfile.Arn" --output text)
CONFIG="Name=$VPC_ORIGIN_NAME,Arn=$INSTANCE_ARN,HTTPPort=8080,OriginProtocolPolicy=http-only"

if [ ! -f /tmp/vpc_origin_id ]; then
    VPC_ORIGIN_ID=$(aws cloudfront list-vpc-origins --output json \
        | jq -r '.Items[] | select(.Name == $VPC_ORIGIN_NAME) | .Id')

    if [ -z "$VPC_ORIGIN_ID" ]; then
        VPC_ORIGIN_ID=$(aws cloudfront create-vpc-origin \
            --vpc-origin-endpoint-config $CONFIG \
            --name $VPC_ORIGIN_NAME \
            --output json | jq -r '.Id')
    fi
    echo $VPC_ORIGIN_ID > /tmp/vpc_origin_id
else
    VPC_ORIGIN_ID=$(cat /tmp/vpc_origin_id)
fi

JSON=$(aws cloudfront get-vpc-origin --id $VPC_ORIGIN_ID \
    --output json)

Arn=$(echo $JSON | jq -r '.VpcOriginEndpointConfig.Arn')
if [ "$Arn" != "$INSTANCE_ARN" ]; then
    aws cloudfront update-vpc-origin \
        --id $VPC_ORIGIN_ID \
        --if-match $(echo $JSON | jq -r '.ETag') \
        --vpc-origin-endpoint-config $CONFIG
fi

JSON=$(aws cloudfront get-distribution --id $CLOUDFRONT_DISTRIBUTION_ID \
    --output json)
# .DistributionConfig.Origins.Items[].VpcOriginConfig.VpcOriginId 중에 $VPC_ORIGIN_ID 가 있는지 확인하자.
VPC_ORIGIN_CONFIG=$(echo $JSON \
    | jq -r '.DistributionConfig.Origins.Items[] | select(.VpcOriginConfig.VpcOriginId == $VPC_ORIGIN_ID)')

if [ -z "$VPC_ORIGIN_CONFIG" ]; then
    JSON=$(echo $JSON \
        | jq '.DistributionConfig.Origins.Items += [{
            "Id": $VPC_ORIGIN_ID,
            "DomainName": $DOMAIN_NAME,
            "OriginPath": "",
            "CustomHeaders": { "Quantity": 0 },
            "VpcOriginConfig": { "VpcOriginId": $VPC_ORIGIN_ID }
        }]')
    aws cloudfront update-distribution \
        --id $CLOUDFRONT_DISTRIBUTION_ID \
        --if-match $(echo $JSON | jq -r '.ETag') \
        --distribution-config $JSON
fi

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
