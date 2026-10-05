#!/usr/bin/env bash
set -euo pipefail

bucket="socialgym-test-media"
queue="person-media-events"
social_notification_queue="social-notification-events.fifo"
queue_arn="arn:aws:sqs:us-east-1:000000000000:${queue}"

awslocal s3api create-bucket --bucket "$bucket"
awslocal sqs create-queue --queue-name "$queue"
awslocal sqs create-queue \
  --queue-name "$social_notification_queue" \
  --attributes FifoQueue=true,ContentBasedDeduplication=false
awslocal s3api put-bucket-notification-configuration \
  --bucket "$bucket" \
  --notification-configuration \
  "{\"QueueConfigurations\":[{\"QueueArn\":\"${queue_arn}\",\"Events\":[\"s3:ObjectCreated:*\"]}]}"
