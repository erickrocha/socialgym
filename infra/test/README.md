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

`start.sh` requires OpenSSL and Docker Compose; it generates the CloudFront
signing key in memory and a short-lived TLS CA/server certificate under
`/tmp/socialgym-test-certs` before starting the stack. Run `./start.sh down` to
stop the stack and remove the temporary TLS key material. `./start.sh down -v`
also removes the disposable database volumes.

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

Timeline runs with `PUSH_PROVIDER_MODE=fake`; it requires no FCM/APNs credentials. To simulate
provider outcomes, set `FAKE_PUSH_PROVIDER_STATUS` to `429`, `500`, `timeout`, `invalid-token`,
`401`, or `configuration`; leave it unset for success. Fake delivery records contain only the
notification UUID and navigation target, never a registration token. Device-token acceptance
tests must verify that registering a token under a different authenticated person transfers its
owner association and prevents delivery to the previous owner. They must also verify that the same
stable `deviceUuid` updates its FCM token after provider rotation, and that successful registration
responses do not return the token.

The PostgreSQL and MongoDB ports are published for host-side tests. The stack
creates no persistent LocalStack state; Postgres and MongoDB data are removed
by `docker compose down -v`.

Stop the test stack and remove its temporary TLS credentials:

```sh
./start.sh down
```

Add `-v` to remove the disposable PostgreSQL and MongoDB volumes too:

```sh
./start.sh down -v
```

The stack uses test-only credentials and ports. Do not use it as a production deployment.

## Running the acceptance tests against this stack

`./start.sh` also loads `seed.sql` (persons/users/settings/consents/friends that the Timeline tests
expect from Workout) once Workout has migrated. Re-run `./start.sh seed` if a Workout acceptance test
refreshed the database.

Timeline `#[ignore]`d tests (Mongo, SQS FIFO, Workout gRPC) run from the host:

```sh
source infra/test/timeline-test-env.sh
cd timeline && cargo test --workspace --all-features -- --include-ignored --test-threads=1
```

The environment points the friendship test at a dedicated queue,
`social-notification-events-acceptance.fifo`, so it does not compete with the running Timeline
service for messages. `./coverage.sh timeline|workout` measures line coverage with the approved
exclusion list (see `01-project_truth/socialgym/product-decisions.md`).
