# SocialGym Test Stack

This isolated Compose stack starts the dependencies required for cross-service acceptance tests:

- PostGIS for `workout` and `integration`
- MongoDB for `timeline`
- LocalStack S3 and SQS, including an S3 object-created notification to the workout media queue
- `workout` REST API
- `integration` consent/person gRPC service
- `timeline` REST API

Start it from this directory:

```sh
./start.sh
```

`start.sh` requires OpenSSL and Docker Compose; it generates the local signing
key in memory before starting the stack.

Endpoints:

- Workout REST: `http://localhost:18090`
- Timeline REST: `http://localhost:18091`
- Integration gRPC TLS: `localhost:18501`
- PostgreSQL: `localhost:55432`
- MongoDB: `localhost:37017`
- LocalStack S3/SQS API: `http://localhost:4566`

The test bucket is `socialgym-test-media`; the SQS queue is `person-media-events`.
The test bucket is `socialgym-test-media`; the SQS queues are `person-media-events` and
`social-notification-events.fifo`. Friendship events use the FIFO queue with the friendship UUID as
`MessageGroupId` and the event UUID as `MessageDeduplicationId`.
The applications reach LocalStack at `http://localstack:4566`. S3 presigned upload
and download URLs are generated against `http://localhost:4566` for host-side test
clients. Their signatures therefore match the host-facing endpoint.

LocalStack Community emulates S3 and SQS, not CloudFront. `start.sh` generates
an ephemeral RSA key in the environment so services can produce CloudFront
signed URL responses; those CloudFront URLs are not served by LocalStack. The
key is not written to disk. Do not use test credentials outside this stack.

Test database credentials:

- PostgreSQL: `workout_test` / `workout_test`, database `workout_test`
- MongoDB app user: `timeline_test` / `timeline_test`, database `timeline_test`
- MongoDB root user: `root_test` / `root_test`

The push test provider is a fake and requires no FCM/APNs credentials. Device-token acceptance
tests must verify that registering a token under a different authenticated person transfers its
owner association and prevents delivery to the previous owner. They must also verify that the same
stable `deviceUuid` updates its FCM token after provider rotation, and that successful registration
responses do not return the token.

The PostgreSQL and MongoDB ports are published for host-side tests. The stack
creates no persistent LocalStack state; Postgres and MongoDB data are removed
by `docker compose down -v`.

Stop and remove the test stack:

```sh
docker compose -f compose.yml down -v
```

The stack uses test-only credentials and ports. Do not use it as a production deployment.
