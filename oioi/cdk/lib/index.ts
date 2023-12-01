import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OioiProps {
    groupName: string;
    image: string;
    vpc?: cdk.aws_ec2.Vpc;
    alb?: cdk.aws_elasticloadbalancingv2.ApplicationLoadBalancer;
    portMappings?: PortMapping[];
}

export class Oioi extends Construct {
    public readonly vpc: cdk.aws_ec2.Vpc;
    public readonly autoScalingGroup: cdk.aws_autoscaling.AutoScalingGroup;

    constructor(scope: Construct, id: string, props: OioiProps) {
        super(scope, id);
        const stack = cdk.Stack.of(this);

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
                init: cdk.aws_ec2.CloudFormationInit.fromConfigSets({
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
                        helloWorld: new cdk.aws_ec2.InitConfig([
                            cdk.aws_ec2.InitCommand.shellCommand(
                                "echo Hello, oioi!",
                            ),
                        ]),
                        setEnv: new cdk.aws_ec2.InitConfig([
                            cdk.aws_ec2.InitCommand.shellCommand(
                                "export EC2_INSTANCE_ID=$(ec2-metadata -i | cut -d ' ' -f 2)",
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
                                    `--log-opt awslogs-group=oioi-agent-${props.groupName}`,
                                    `--log-opt awslogs-stream=oioi-agent-${props.groupName}-$EC2_INSTANCE_ID`,
                                    "--log-opt awslogs-create-group=true",

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
                                                        hostPort ??
                                                        containerPort
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
                }),
                signals: cdk.aws_autoscaling.Signals.waitForMinCapacity(),
                updatePolicy:
                    cdk.aws_autoscaling.UpdatePolicy.replacingUpdate(),
                vpcSubnets: {
                    subnetType: cdk.aws_ec2.SubnetType.PUBLIC,
                },
                healthCheck: cdk.aws_autoscaling.HealthCheck.elb({
                    grace: cdk.Duration.seconds(300),
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
                    },
                }),
            },
        );

        new cdk.aws_ssm.StringParameter(this, "ImageParameter", {
            parameterName: `/oioi/${props.groupName}/image`,
            stringValue: props.image,
        });

        // TODO: Add cloudwatch dashboard
    }
}

type PortMapping = {
    containerPort: number;
    hostPort: number;
    protocol: "tcp" | "udp";
};
