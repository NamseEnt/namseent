import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OioiProps {
    groupName: string;
    image: cdk.aws_ecr_assets.DockerImageAsset | OioiDockerNormalImage;
    vpc?: cdk.aws_ec2.Vpc;
    alb?: cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer;
    portMappings?: PortMapping[];
    /**
     * @default cdk.RemovalPolicy.DESTROY
     * */
    logRemovalPolicy?: cdk.RemovalPolicy;
    /**
     * @default cdk.aws_logs.RetentionDays.ONE_WEEK
     * */
    logRetention?: cdk.aws_logs.RetentionDays;
}

export class Oioi extends Construct {
    public readonly vpc: cdk.aws_ec2.Vpc;
    public readonly alb: cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer;
    public readonly autoScalingGroup: cdk.aws_autoscaling.AutoScalingGroup;

    constructor(scope: Construct, id: string, props: OioiProps) {
        super(scope, id);
        const stackUuid = cdk.Fn.select(
            2,
            cdk.Fn.split("/", `${cdk.Aws.STACK_ID}`),
        );

        const vpc = (this.vpc =
            props.vpc ??
            new cdk.aws_ec2.Vpc(this, "Vpc", {
                natGateways: 0,
                restrictDefaultSecurityGroup: false,
            }));

        this.alb =
            props.alb ??
            new cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer(
                this,
                "Alb",
                {
                    vpc,
                    internetFacing: true,
                },
            );

        const systemMessagesLogGroup = new cdk.aws_logs.LogGroup(
            this,
            "SystemMessagesLogGroup",
            {
                logGroupName: `/oioi/${props.groupName}/system_messages-${stackUuid}`,
                retention:
                    props.logRetention ?? cdk.aws_logs.RetentionDays.ONE_WEEK,
                removalPolicy:
                    props.logRemovalPolicy ?? cdk.RemovalPolicy.RETAIN,
            },
        );

        const agentLogGroup = new cdk.aws_logs.LogGroup(this, "AgentLogGroup", {
            logGroupName: `/oioi/${props.groupName}/agent-${stackUuid}`,
            retention:
                props.logRetention ?? cdk.aws_logs.RetentionDays.ONE_WEEK,
            removalPolicy: props.logRemovalPolicy ?? cdk.RemovalPolicy.RETAIN,
        });

        const imageUri = (() => {
            const image = props.image;

            // I Have no idea why instanceof cdk.aws_ecr_assets.DockerImageAsset is not working
            if ("imageUri" in image) {
                return image.imageUri;
            }
            if (image instanceof OioiDockerNormalImage) {
                return image.uri;
            }
            throw new Error("Invalid image");
        })();

        const dockerLoginScript = (() => {
            const image = props.image;
            if ("imageUri" in image) {
                return `aws ecr get-login-password | docker login --username AWS --password-stdin ${image.repository.repositoryUri}`;
            }
            return "";
        })();

        const imageParameter = new cdk.aws_ssm.StringParameter(
            this,
            "ImageParameter",
            {
                parameterName: `/oioi/${props.groupName}/container-config`,
                stringValue: JSON.stringify({
                    imageUri,
                    portMappings: props.portMappings,
                    dockerLoginScript,
                    updatedAt: new Date().toISOString(),
                }),
            },
        );

        const autoScalingGroup = (this.autoScalingGroup =
            new cdk.aws_autoscaling.AutoScalingGroup(this, "ASG", {
                vpc,
                instanceType: cdk.aws_ec2.InstanceType.of(
                    cdk.aws_ec2.InstanceClass.T4G,
                    cdk.aws_ec2.InstanceSize.MICRO,
                ),
                machineImage: cdk.aws_ec2.MachineImage.latestAmazonLinux2023({
                    cpuType: cdk.aws_ec2.AmazonLinuxCpuType.ARM_64,
                }),
                associatePublicIpAddress: true,
                updatePolicy:
                    cdk.aws_autoscaling.UpdatePolicy.replacingUpdate(),
                vpcSubnets: {
                    subnetType: cdk.aws_ec2.SubnetType.PUBLIC,
                },
                healthCheck: cdk.aws_autoscaling.HealthCheck.elb({
                    grace: cdk.Duration.minutes(10),
                }),
                role: new cdk.aws_iam.Role(this, "Role", {
                    assumedBy: new cdk.aws_iam.ServicePrincipal(
                        "ec2.amazonaws.com",
                    ),
                    managedPolicies: [
                        cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName(
                            "AmazonSSMManagedInstanceCore",
                        ),
                        cdk.aws_iam.ManagedPolicy.fromAwsManagedPolicyName(
                            "CloudWatchAgentServerPolicy",
                        ),
                    ],
                    inlinePolicies: {
                        ecr: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    actions: [
                                        "ecr-public:GetAuthorizationToken",
                                        "ecr:GetAuthorizationToken",
                                        "ecr:BatchGetImage",
                                        "ecr:BatchCheckLayerAvailability",
                                        "ecr:GetDownloadUrlForLayer",
                                        "sts:GetServiceBearerToken",
                                    ],
                                    resources: ["*"],
                                }),
                            ],
                        }),
                    },
                }),
                userData: (() => {
                    const userData = cdk.aws_ec2.UserData.forLinux();
                    userData.addCommands(
                        `export EC2_INSTANCE_ID=$(ec2-metadata -i | cut -d ' ' -f 2)`,
                        `export GROUP_NAME=${props.groupName}`,

                        `cat <<EOF > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json
{
    "logs": {
        "logs_collected": {
            "files": {
                "collect_list": [
                    {
                        "file_path": "/var/log/messages",
                        "log_group_name": "${systemMessagesLogGroup.logGroupName}",
                        "log_stream_name": "{instance_id}-/var/log/messages/"
                    }
                ]
            }
        }
    }
}

EOF
`,
                        `yum install -y amazon-cloudwatch-agent`,
                        `\
/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
-a fetch-config \
-m ec2 -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json \
-s`,

                        `echo Hello, oioi! EC2_INSTANCE_ID = $EC2_INSTANCE_ID`,

                        `yum install -y docker`,
                        `systemctl enable docker`,
                        `systemctl start docker`,

                        `\
aws ecr-public get-login-password --region us-east-1 | \
docker login --username AWS --password-stdin public.ecr.aws
`,

                        `docker run ${[
                            "-d",
                            "--restart always",
                            "--name oioi-agent",

                            "--log-driver awslogs",
                            `--log-opt awslogs-group=${agentLogGroup.logGroupName}`,
                            `--log-opt awslogs-stream=$EC2_INSTANCE_ID`,

                            `-e GROUP_NAME=${props.groupName}`,
                            `-e EC2_INSTANCE_ID=$EC2_INSTANCE_ID`,

                            "-v /var/run/docker.sock:/var/run/docker.sock",
                        ].join(
                            " ",
                        )} public.ecr.aws/o4b6l4b3/oioi:latest ./oioi-agent`,
                    );

                    return userData;
                })(),
            }));

        autoScalingGroup.node.addDependency(
            systemMessagesLogGroup,
            agentLogGroup,
            imageParameter,
        );

        // TODO: Add cloudwatch dashboard
    }
}

type PortMapping = {
    containerPort: number;
    hostPort: number;
    protocol: "tcp" | "udp";
};

export class OioiDockerNormalImage {
    public readonly uri: string;

    constructor(uri: string) {
        this.uri = uri;
    }
}
