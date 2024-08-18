FROM public.ecr.aws/lambda/provided.al2023

RUN dnf install -y certbot python3-certbot-dns-route53

COPY bootstrap /var/runtime/bootstrap
COPY function.sh /var/runtime/function.sh
