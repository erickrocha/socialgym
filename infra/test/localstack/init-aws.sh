#!/usr/bin/env bash
set -euo pipefail

bucket="socialgym-test-media"
queue="person-media-events"
social_notification_queue="social-notification-events.fifo"
# Dedicated to the Timeline acceptance tests so they never compete with the running timeline service.
acceptance_queue="social-notification-events-acceptance.fifo"
queue_arn="arn:aws:sqs:us-east-1:000000000000:${queue}"

awslocal s3api create-bucket --bucket "$bucket"
awslocal sqs create-queue --queue-name "$queue"
awslocal sqs create-queue \
  --queue-name "$social_notification_queue" \
  --attributes FifoQueue=true,ContentBasedDeduplication=false
awslocal sqs create-queue \
  --queue-name "$acceptance_queue" \
  --attributes FifoQueue=true,ContentBasedDeduplication=false
awslocal s3api put-bucket-notification-configuration \
  --bucket "$bucket" \
  --notification-configuration \
  "{\"QueueConfigurations\":[{\"QueueArn\":\"${queue_arn}\",\"Events\":[\"s3:ObjectCreated:*\"]}]}"
# The web client uploads straight to the bucket with a pre-signed PUT from the browser, which needs CORS
# (the Playwright run serves the web client on localhost:5173).
awslocal s3api put-bucket-cors \
  --bucket "$bucket" \
  --cors-configuration \
  '{"CORSRules":[{"AllowedOrigins":["http://localhost:5173"],"AllowedMethods":["PUT","GET","HEAD"],"AllowedHeaders":["*"],"ExposeHeaders":["ETag"],"MaxAgeSeconds":3000}]}'
