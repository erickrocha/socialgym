# billion/ — production-like stack

Docker Compose stack: nginx gateway, PostgreSQL/PostGIS, MongoDB, and the three Rust
services (`workout-app`, `integration-app` — the Tonic gRPC server,
`timeline-app`). `development/` is the equivalent stack for local dev; this
one is meant to run closer to how a real deployment looks.

## First-time setup

1. Copy the env template and fill in every `CHANGE_ME`:
   ```bash
   cp .env.example .env
   ```
   - `POSTGRES_*`/`MONGO_*` passwords and `AUTH_ACCESS_TOKEN_SECRET`/`AUTH_REFRESH_TOKEN_SECRET`
     are fully self-managed — generate them yourself, e.g.:
     ```bash
     openssl rand -base64 96 | tr -d '\n'   # JWT secrets
     openssl rand -base64 24 | tr -dc 'A-Za-z0-9'   # DB passwords
     ```
   - `AUTH_ACCESS_TOKEN_SECRET`/`AUTH_REFRESH_TOKEN_SECRET` are a single shared pair
     consumed by all three Rust services (`workout-app`, `integration-app`,
     `timeline-app`). They **must** be identical everywhere — `workout-app` signs the
     JWT at `/login` and the other two validate it, so a mismatch makes every
     authenticated cross-service call (e.g. `GET /timeline/api/feed`) return 401.
   - `*_AWS_ACCESS_KEY_ID`/`*_AWS_SECRET_ACCESS_KEY`, `*_CLOUDFRONT_KEY_PAIR_ID`,
     `*_PRIVATE_KEY_RAW` come from your AWS account (IAM + CloudFront key
     group) — not something to generate locally.
   - Keep `WORKOUT_DATABASE_URL`/`TIMELINE_DATABASE_URL`'s embedded password
     in sync with `POSTGRES_PASSWORD`/`MONGODB_PASSWORD` respectively.
   - **Never commit `.env`.**

2. TLS certs: `../certs/server.crt`/`server.key` must exist. The private key
   is gitignored (not committed) — generate your own from `../certs/san.cnf`:
   ```bash
   openssl req -x509 -nodes -newkey rsa:2048 -days 825 \
     -keyout ../certs/server.key -out ../certs/server.crt \
     -config ../certs/san.cnf -extensions v3_req
   ```
   Re-run this (edit `../certs/san.cnf` first) if the SAN list needs to
   change — e.g. a new ngrok hostname. Both `gateway` and `integration-app`
   mount these read-only; `timeline-app` mounts only the cert (it's a gRPC
   client, not a server). This is a self-signed cert — for a real public
   deployment with a trusted CA-issued certificate, swap this for a Let's
   Encrypt/ACME-issued cert instead.

3. Build and start:
   ```bash
   docker compose build
   docker compose up -d
   docker compose ps
   ```

4. Run the compliance gate before every production release:
   ```bash
   bash verify-compliance-gate.sh
   ```
   The automated gate rejects legal placeholders and missing region/consent
   configuration. The signed evidence listed in `docs/compliance/` remains a
   manual release requirement.

## Network model

Only `gateway` publishes ports to the host (`80`, `443`). Everything else —
PostgreSQL/PostGIS, MongoDB, `workout-app`, `integration-app`, `timeline-app` — is
reachable only from other containers on the internal Docker networks, by
service name. There are two networks:

- **`edge`** — `gateway` + the three app services. Has normal outbound
  internet access (the apps need it for AWS S3/CloudFront/SQS calls).
- **`data`** (`internal: true`, no internet egress) — PostgreSQL/PostGIS, MongoDB, and
  the three app services. `gateway` is deliberately not on this network, so
  a compromised nginx can't reach either database directly.

All HTTP/gRPC traffic from outside comes in through nginx on 443
(`nginx.conf` proxies `/login`, `/signup`, `/workout/api/`, `/timeline/api/`
to the right app, and `/grpc.*` to `integration-app`'s gRPC port via
`grpc_pass`). There's no need to publish `50051`, `8090`, `8091`, `5432`, or
`27017` to the host for normal operation.

### Inspecting the databases from the host (DBeaver, Compass)

`compose.db-access.yml` is an opt-in overlay that publishes PostgreSQL on host
port `5433` and MongoDB on `27018` (offset from `infra/dev/`'s `5432`/`27017`
so both stacks can run at once). It also attaches `postgres`/`mongo` to `edge`,
which Docker requires before it will publish a host port for a container that is
otherwise only on the `internal: true` `data` network.

```bash
docker compose -f compose.yml -f compose.db-access.yml up -d
```

- **DBeaver / PostgreSQL** — `localhost:5433`, database `workout_prod`, user
  `workout`, password `POSTGRES_PASSWORD` from `.env`.
- **MongoDB Compass** —
  `mongodb://root:<MONGO_ROOT_PASSWORD>@localhost:27018/?authSource=admin&directConnection=true`.

Plain `docker compose ...` (no `-f compose.db-access.yml`) leaves the databases
unpublished. Don't use this overlay for a real deployment.

## Known limitations

- `integration-app` runs as root in its container — it's the one service
  that needs to read `server.key` (mode 600) at startup, same as nginx
  itself already necessarily does. Not treated as an oversight; loosening
  the key's file permissions just to run this one service rootless would be
  a worse trade.
- The TLS cert is self-signed with an ngrok hostname baked into its SAN —
  fine for tunneling, not suitable for a public deployment with untrusted
  clients.
- No CI/CD — images are built locally via `docker compose build`. If you
  want an automated build/push pipeline, that's a separate piece of work.
