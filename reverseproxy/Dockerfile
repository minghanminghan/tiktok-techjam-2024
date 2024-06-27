FROM nginx:latest

# Copy custom configuration file from the host to the container
COPY conf.d/default.conf /etc/nginx/conf.d/default.conf

# Copy SSL certificates (if needed)
COPY ssl /etc/nginx/ssl

# Expose ports 80 and 443
EXPOSE 80
EXPOSE 443

