import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OioiProps {
    groupName: string;
    image:
        | {
              type: "ecr";
              account: string;
              region: string;
              repository: string;
              tag: string;
          }
        | {
              type: "normal";
              /**
               * @example public.ecr.aws/o4b6l4b3/oioi:latest
               */
              uri: string;
          };
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

        const alb = (this.alb =
            props.alb ??
            new cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer(
                this,
                "Alb",
                {
                    vpc,
                    internetFacing: true,
                },
            ));

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
            if (image.type === "ecr") {
                return `${image.account}.dkr.ecr.${image.region}.amazonaws.com/${image.repository}:${image.tag}`;
            } else {
                return image.uri;
            }
        })();

        const imageParameter = new cdk.aws_ssm.StringParameter(
            this,
            "ImageParameter",
            {
                parameterName: `/oioi/${props.groupName}/image`,
                stringValue: imageUri,
            },
        );

        const dockerLoginScript = (() => {
            const image = props.image;
            if (image.type === "ecr") {
                return `aws ecr get-login-password --region ${image.region} | docker login --username AWS --password-stdin ${image.account}.dkr.ecr.${image.region}.amazonaws.com`;
            } else {
                return "";
            }
        })();

        const initConfigs: Record<string, cdk.aws_ec2.InitConfig> = {
            setEnv: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitCommand.shellCommand("ec2-metadata -i"),
                cdk.aws_ec2.InitCommand.shellCommand(
                    "echo $(ec2-metadata -i | cut -d ' ' -f 2)",
                ),
            ]),
            awslogs: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitFile.fromString(
                    "/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json",
                    `
{
    "logs": {
        "logs_collected": {
            "files": {
                "collect_list": [
                    {
                        "file_path": "/var/log/messages",
                        "log_group_name": "${systemMessagesLogGroup.logGroupName}",
                        "log_stream_name": "{instance_id}-/var/log/messages/"
                    },
                    {
                        "file_path": "/var/log/cfn-init-cmd.log",
                        "log_group_name": "${systemMessagesLogGroup.logGroupName}",
                        "log_stream_name": "{instance_id}-/var/log/cfn-init-cmd.log"
                    },
                    {
                        "file_path": "/var/log/cfn-init.log",
                        "log_group_name": "${systemMessagesLogGroup.logGroupName}",
                        "log_stream_name": "{instance_id}-/var/log/cfn-init.log"
                    }
                ]
            }
        }
    }       
}
`,
                ),
                cdk.aws_ec2.InitPackage.yum("amazon-cloudwatch-agent"),
                cdk.aws_ec2.InitCommand.shellCommand(
                    `/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl -a fetch-config -m ec2 -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json -s`,
                ),
            ]),
            helloWorld: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitCommand.shellCommand(
                    `echo Hello, oioi! EC2_INSTANCE_ID = $EC2_INSTANCE_ID`,
                ),
            ]),
            docker: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitPackage.yum("docker"),
                cdk.aws_ec2.InitService.enable("docker"),
            ]),
            ecrLogin: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitCommand.shellCommand(
                    `
aws ecr-public get-login-password --region us-east-1 |
docker login --username AWS --password-stdin public.ecr.aws
`.replaceAll("\n", " "),
                ),
            ]),
            runAgent: new cdk.aws_ec2.InitConfig([
                cdk.aws_ec2.InitCommand.shellCommand(
                    `docker run ${[
                        "-d",
                        "--restart always",
                        "--name oioi-agent",

                        "--log-driver awslogs",
                        `--log-opt awslogs-group=${agentLogGroup.logGroupName}`,
                        `--log-opt awslogs-stream=$EC2_INSTANCE_ID`,

                        `-e GROUP_NAME=${props.groupName}`,
                        `-e EC2_INSTANCE_ID=$(ec2-metadata -i | cut -d ' ' -f 2)`,
                        `-e PORT_MAPPINGS=${
                            props.portMappings
                                ?.map(
                                    ({ containerPort, hostPort, protocol }) =>
                                        `${
                                            hostPort ?? containerPort
                                        }:${containerPort}/${protocol}`,
                                )
                                .join(",") ?? ""
                        }`,
                        `-e DOCKER_LOGIN_SCRIPT=${dockerLoginScript}`,

                        "-v /var/run/docker.sock:/var/run/docker.sock",
                    ].join(
                        " ",
                    )} public.ecr.aws/o4b6l4b3/oioi:latest ./oioi-agent`,
                ),
            ]),
        };

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
                signals: cdk.aws_autoscaling.Signals.waitForMinCapacity(),
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
                init: cdk.aws_ec2.CloudFormationInit.fromConfigSets({
                    configSets: {
                        default: Object.keys(initConfigs),
                    },
                    configs: initConfigs,
                }),
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
