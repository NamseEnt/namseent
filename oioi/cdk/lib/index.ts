import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OioiProps {
    groupName: string;
    image: string;
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
    public readonly autoScalingGroup: cdk.aws_autoscaling.AutoScalingGroup;

    constructor(scope: Construct, id: string, props: OioiProps) {
        super(scope, id);
        const stack = cdk.Stack.of(this);
        const stackUuid = cdk.Fn.select(
            2,
            cdk.Fn.split("/", `${cdk.Aws.STACK_ID}`),
        );

        this.vpc =
            props.vpc ??
            new cdk.aws_ec2.Vpc(this, "Vpc", {
                natGateways: 0,
                restrictDefaultSecurityGroup: false,
            });

        const alb =
            props.alb ??
            new cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer(
                this,
                "Alb",
                {
                    vpc: this.vpc,
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

        const imageParameter = new cdk.aws_ssm.StringParameter(
            this,
            "ImageParameter",
            {
                parameterName: `/oioi/${props.groupName}/image`,
                stringValue: props.image,
            },
        );

        const init = cdk.aws_ec2.CloudFormationInit.fromConfigSets({
            configSets: {
                default: [
                    "helloWorld",
                    "setEnv",
                    "docker",
                    "runAgent",
                    "verifyInstanceHealth",
                ],
            },
            configs: {
                setEnv: new cdk.aws_ec2.InitConfig([
                    cdk.aws_ec2.InitCommand.shellCommand(
                        "export EC2_INSTANCE_ID=$(ec2-metadata -i | cut -d ' ' -f 2)",
                    ),
                ]),
                awslogs: new cdk.aws_ec2.InitConfig([
                    cdk.aws_ec2.InitFile.fromString(
                        "/etc/awslogs/awslogs.conf",
                        `
[general]
state_file = /var/lib/awslogs/agent-state
use_gzip_http_content_encoding = true

[/var/log/messages]
file = /var/log/messages
log_group_name = ${systemMessagesLogGroup.logGroupName}
log_stream_name = {instance_id}-/var/log/messages/

[/var/log/cfn-init-cmd.log]
file = /var/log/cfn-init-cmd.log
log_group_name = ${systemMessagesLogGroup.logGroupName}
log_stream_name = {instance_id}-/var/log/cfn-init-cmd.log

[/var/log/cfn-init.log]
file = /var/log/cfn-init.log
log_group_name = ${systemMessagesLogGroup.logGroupName}
log_stream_name = {instance_id}-/var/log/cfn-init.log
`,
                    ),
                    cdk.aws_ec2.InitPackage.yum("awslogs"),
                    cdk.aws_ec2.InitService.enable("awslogs"),
                ]),
                helloWorld: new cdk.aws_ec2.InitConfig([
                    cdk.aws_ec2.InitCommand.shellCommand(
                        `echo Hello, oioi! EC2_INSTANCE_ID = $EC2_INSTANCE_ID`,
                    ),
                ]),
                docker: new cdk.aws_ec2.InitConfig([
                    cdk.aws_ec2.InitPackage.yum("docker"),
                    cdk.aws_ec2.InitService.enable("docker"),
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
                            `-e EC2_INSTANCE_ID=$EC2_INSTANCE_ID`,
                            `-e PORT_MAPPINGS=${
                                props.portMappings
                                    ?.map(
                                        ({
                                            containerPort,
                                            hostPort,
                                            protocol,
                                        }) =>
                                            `${
                                                hostPort ?? containerPort
                                            }:${containerPort}/${protocol}`,
                                    )
                                    .join(",") ?? ""
                            }`,
                            "-v /var/run/docker.sock:/var/run/docker.sock",
                        ].join(
                            " ",
                        )} public.ecr.aws/o4b6l4b3/oioi:latest ./oioi-agent`,
                    ),
                ]),
                verifyInstanceHealth: new cdk.aws_ec2.InitConfig([
                    cdk.aws_ec2.InitCommand.shellCommand(
                        `
until [ "$state" == "\\"InService\\"" ]; do state=$(aws --region ${stack.region} elb describe-target-health
--load-balancer-name ${alb.loadBalancerName}
--instances $EC2_INSTANCE_ID
--query InstanceStates[0].State); sleep 10; done`.replaceAll("\n", " "),
                    ),
                ]),
            },
        });

        this.autoScalingGroup = new cdk.aws_autoscaling.AutoScalingGroup(
            this,
            "ASG",
            {
                vpc: this.vpc,
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
                        cloudFormationHelper: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    actions: ["cloudformation:SignalResource"],
                                    resources: [
                                        `arn:aws:cloudformation:${stack.region}:${stack.account}:stack/${stack.stackName}/*`,
                                    ],
                                }),
                            ],
                        }),
                        publicEcr: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    actions: [
                                        "ecr-public:GetAuthorizationToken",
                                    ],
                                    resources: ["*"],
                                }),
                                new cdk.aws_iam.PolicyStatement({
                                    actions: ["sts:GetServiceBearerToken"],
                                    resources: ["*"],
                                }),
                            ],
                        }),
                    },
                }),
            },
        );

        // `applyCloudFormationInit` for verbose logs
        this.autoScalingGroup.applyCloudFormationInit(init);

        this.autoScalingGroup.node.addDependency(
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
