#!/usr/bin/env bash
set -euo pipefail

bucket="socialgym-test-media"
queue="person-media-events"
queue_arn="arn:aws:sqs:us-east-1:000000000000:${queue}"

awslocal s3api create-bucket --bucket "$bucket"
awslocal sqs create-queue --queue-name "$queue"
awslocal s3api put-bucket-notification-configuration \
  --bucket "$bucket" \
  --notification-configuration \
  "{\"QueueConfigurations\":[{\"QueueArn\":\"${queue_arn}\",\"Events\":[\"s3:ObjectCreated:*\"]}]}"
