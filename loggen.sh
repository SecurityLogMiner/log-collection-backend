#!/bin/bash

LOG_FILE="/var/log/logminer/logs/security_logs.log"

# generate a random security log entry
generate_log_entry() {
    TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
    EVENT_ID=$((RANDOM % 1000 + 1000))
    SOURCE="Security"
    MESSAGE="Random security log message with ID $EVENT_ID"
    echo "$TIMESTAMP - $SOURCE - Event ID: $EVENT_ID - $MESSAGE"
}

# infinite loop to generate logs periodically
while true; do
    LOG_ENTRY=$(generate_log_entry)
    echo "$LOG_ENTRY" >> "$LOG_FILE"
    sleep 30  # sleep for 30 seconds before the next log is generated
done

# directions:

# chmod +x loggen.sh

# command to run the script in the background
# nohup ./loggen.sh &
