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

        this.vpc =
            props.vpc ??
            new cdk.aws_ec2.Vpc(this, "Vpc", {
                natGateways: 0,
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
                    userData: getUserData(props),
                }),
                associatePublicIpAddress: true,
            },
        );

        new cdk.aws_ssm.StringParameter(this, "ImageParameter", {
            parameterName: `/oioi/${props.groupName}/image`,
            stringValue: props.image,
        });

        // TODO: Add cloudwatch
    }
}

type PortMapping = {
    containerPort: number;
    hostPort: number;
    protocol: "tcp" | "udp";
};

function getUserData({
    groupName,
    portMappings,
}: OioiProps): cdk.aws_ec2.UserData {
    const userData = cdk.aws_ec2.UserData.forLinux();
    const portMappingsString =
        portMappings
            ?.map(
                ({ containerPort, hostPort, protocol }) =>
                    `${hostPort ?? containerPort}:${containerPort}/${protocol}`,
            )
            .join(",") ?? "";

    const dockerOptions = [
        "-d",
        "--name oioi-agent",

        "--log-driver awslogs",
        `--log-opt awslogs-group=oioi-agent-${groupName}`,
        `--log-opt awslogs-stream=oioi-agent-${groupName}-$EC2_INSTANCE_ID`,
        "--log-opt awslogs-create-group=true",

        `-e GROUP_NAME=${groupName}`,
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
        `docker run -d ${dockerOptions} public.ecr.aws/namseent/oioi ./oioi-agent`,
    );

    return userData;
}
