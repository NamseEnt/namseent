function handler() {
    EVENT_DATA=$1
    echo "$EVENT_DATA" 1>&2
    RESPONSE="Echoing request: '$EVENT_DATA'"

    certbot certonly --dns-route53 -d visual-novel.namseent.com --email projectluda@gmail.com --non-interactive --agree-tos

    aws s3 sync /etc/letsencrypt/live/visual-novel.namseent.com s3://$BUCKET_NAME/certs

    echo $RESPONSE
}
