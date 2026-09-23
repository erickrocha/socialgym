# SocialGym Test Stack

This isolated Compose stack starts the dependencies required for cross-service acceptance tests:

- PostGIS for `workout` and `integration`
- MongoDB for `timeline`
- `workout` REST API
- `integration` consent/person gRPC service
- `timeline` REST API

Start it from this directory:

```sh
docker compose -f compose.yml up --build -d
```

Endpoints:

- Workout REST: `http://localhost:18090`
- Timeline REST: `http://localhost:18091`
- Integration gRPC TLS: `localhost:18501`
- PostgreSQL: `localhost:55432`
- MongoDB: `localhost:37017`

Stop and remove the test stack:

```sh
docker compose -f compose.yml down -v
```

The stack uses test-only credentials and ports. Do not use it as a production deployment.
