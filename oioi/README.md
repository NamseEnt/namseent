# oioi

`O`ne `I`nstance `O`ne `I`mage

oioi is the docker orchestration system.

Unlike kubernetes, oioi doesn't abstract instance layer.

You can set the map between instance type and image, and the amount of desired number of instances.

Autoscaling and monitoring are also supported.

# oioi agent

oioi agent is the simple rust program which manage instance's running container.

oioi agent tries to synchronize the container's image to ensure it is up-to-date by fetching newest container image information from oioi store periodically.

# oioi store

oioi store is the simple store which save the container configuration.

For now, oioi system saves configuration in AWS SSM.

# Auto scaling

oioi doesn't provide auto scaling feature directly. It uses AWS Ec2 Auto Scaling.

# How it works

1. create oioi-agent image to ecr with cdk
2. run new ec2 instance with ec2 auto scaling
3. put user-data to ec2 instance to run oioi-agent
4. oioi-agent checks configuration updates and runs docker container

# Project Motivation

Have you ever deployed and managed servers using Fargate, EC2, Lambda, Kubernetes, or similar technologies? I recently had the experience of creating such deployments through a few freelance projects, and personally, it was quite an ordeal.

Services like AWS ECS or Kubernetes are not designed for small businesses like my clients. These businesses might only need a single instance, and scaling is necessary only during sudden spikes in traffic. However, working with ECS can be anything but straightforward. It's not that it's impossible; it's just not easy. You have to set memory limits, and if that limit is not half of an instance, deployment fails. On top of that, system memory needs to be subtracted. It's genuinely frustrating.

Everything is complex, with a steep learning curve. Dealing with it often felt like the system was mocking me, saying, "You're so dumb, lol." Struggling to overcome errors, I sometimes found myself sinking into a sense of inferiority, questioning if my struggles were due to my own shortcomings.

I wanted to eliminate the darn instance layer abstraction they provided. Let's just have one instance for one container. It's very similar to Elastic Beanstalk, but have you tried using EB? It's just as frustrating. I don't want to deal with their CLI; I just want to use CDK. I really dislike EB!

Perhaps, as I develop this, it might end up resembling EB. However, I aspire to create something better than what's currently available. That's why I embarked on this challenge.

This project is not aimed at presenting a perfect system. Instead, it's a reflection of my struggles with server deployment. It might be a futile attempt, and I'm not seeking success. Instead, I'll enjoy the process for my own satisfaction.