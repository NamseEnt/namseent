FROM nginx:alpine

RUN echo "server { listen 80; location / { return 200 $OUTPUT; } }" > /etc/nginx/conf.d/default.conf

CMD ["nginx", "-g", "daemon off;"]