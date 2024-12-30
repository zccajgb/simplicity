FROM ubuntu:latest

# Install cron
RUN apt-get update && apt-get install -y cron

# Set working directory
WORKDIR /app

# Copy script and cron file into the container
COPY script.sh /app/script.sh
COPY Crontab /etc/cron.d/Crontab

# Make script executable and set permissions for cron file
RUN chmod +x /app/script.sh && chmod 0644 /etc/cron.d/Crontab

# Apply the cron job
RUN crontab /etc/cron.d/Crontab

# Create log file and start cron in foreground
RUN touch /var/log/cron.log
CMD ["cron", "-f"]
