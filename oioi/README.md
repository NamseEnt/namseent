# oioi

`O`ne `I`nstance `O`ne `I`mage

oioi is the docker orchestration system.

Unlike kubernetes, oioi doesn't abstract instance layer.

You can set the map between instance type and image, and the amount of desired number of instances.

Autoscaling and monitoring are also supported.

# oioi AMI(Amazon machine image)

oioi AMI is light machine image which contains oioi agent as daemon.

# oioi agent

oioi agent is the simple rust program which manage instance's running container.

oioi agent tries to synchronize the container's image to ensure it is up-to-date by fetching newest container image information from oioi store periodically.

# oioi store

oioi store is the simple store which save the newest docker image information.

For now, oioi system saves docker images in AWS ECR.

# Auto scaling

oioi doesn't provide auto scaling feature directly. It uses AWS Ec2 Auto Scaling.

# How it works

1. create oioi-agent image to ecr with cdk
2. run new ec2 instance with ec2 auto scaling
3. put user-data to ec2 instance to run oioi-agent
