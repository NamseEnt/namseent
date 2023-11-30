import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OioiProps {
    groupName: string;
    image: string;
    vpc?: cdk.aws_ec2.Vpc;
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

        const userData = cdk.aws_ec2.UserData.forLinux();
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
                    userData,
                }),
                associatePublicIpAddress: true,
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

        const portMappingsString =
            props.portMappings
                ?.map(
                    ({ containerPort, hostPort, protocol }) =>
                        `${
                            hostPort ?? containerPort
                        }:${containerPort}/${protocol}`,
                )
                .join(",") ?? "";

        const dockerOptions = [
            "-d",
            "--restart always",
            "--name oioi-agent",

            "--log-driver awslogs",
            `--log-opt awslogs-group=oioi-agent-${props.groupName}`,
            `--log-opt awslogs-stream=oioi-agent-${props.groupName}-$EC2_INSTANCE_ID`,
            "--log-opt awslogs-create-group=true",

            `-e GROUP_NAME=${props.groupName}`,
            `-e EC2_INSTANCE_ID=$EC2_INSTANCE_ID`,
            `-e PORT_MAPPINGS=${portMappingsString}`,
            "-v /var/run/docker.sock:/var/run/docker.sock docker",
        ].join(" ");

        userData.addCommands(
            "echo Hello, oioi!",
            "export EC2_INSTANCE_ID=$(ec2-metadata -i | cut -d ' ' -f 2)",
            "yum install -y docker",
            "systemctl start docker",
            "systemctl enable docker",
            `docker run ${dockerOptions} public.ecr.aws/o4b6l4b3/oioi:latest ./oioi-agent`,
            `/opt/aws/bin/cfn-signal -e $? --stack ${
                stack.stackName
            } --resource ${stack.getLogicalId(
                this.autoScalingGroup.node.defaultChild as cdk.CfnElement,
            )} --region ${stack.region}`,
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
