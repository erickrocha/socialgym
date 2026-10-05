#!/usr/bin/env bash
# LocalStack settings for host-side acceptance tests (Timeline and Workout). Sourced by
# timeline-test-env.sh and coverage.sh. Requires `./start.sh` to be up.
export AWS_ENDPOINT_URL="http://localhost:4566"
export AWS_REGION="us-east-1"
export AWS_ACCESS_KEY_ID="test"
export AWS_SECRET_ACCESS_KEY="test"
# Dedicated queue (not the one the running timeline service consumes).
export AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL="http://localhost:4566/000000000000/social-notification-events-acceptance.fifo"
