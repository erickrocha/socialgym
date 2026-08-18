# SocialGym

SocialGym is a social network built around wellness: people share posts and
photos, plan and track workouts, and (in progress) discover and connect with
fitness professionals and companies.

There are three actor types in the domain model:

- **Person** — the personal profile. Social features (posts, comments,
  reactions), training (workouts, exercises, progress tracking), sharing
  content with friends, and discovering professionals/companies.
- **Business Profile — Professional** — a person offering wellness expertise
  (personal trainer, nutritionist, etc.). One person can hold several.
- **Business Profile — Company** — e.g. a gym; can have many Professional
  profiles and many persons associated with it.

Authority between these actors is always delegated by explicit consent, never
implied by association — the same request/accept pattern already used by
`friends`. Several pieces of this model (Professional↔Person links,
Company↔Professional links, permission grants, the professionals/companies
marketplace) are on the roadmap but not implemented yet — see `CLAUDE.md` for
the current built-vs-intended breakdown.

## Repository layout

| Path | What it is |
|---|---|
| `workout/` | Rust/Axum REST API + Tonic gRPC service. Source of truth for Person, User, Friends, Workout/Exercise, and Business Profile data (PostgreSQL/PostGIS). |
| `timeline/` | Rust/Axum REST API. Owns Post/Feed, Reactions, Comments, Notifications, Evolution Check-ins (MongoDB). Reads Person/Friend/User data from `workout` over gRPC. |
| `socialgym_web/` | React 19 + Vite + Redux Toolkit web frontend. |
| `socialgym_mobile/` | Flutter app for SocialGym itself. iOS and Android only. |
| `lapidation_mobile/` | Independent Flutter client for a separate brand ("Lapidation Clinic"), derived from the SocialGym mobile feature set. iOS and Android only. |
| `infra/dev/` | Docker Compose stack for local development — gateway (nginx), PostgreSQL/PostGIS, MongoDB. The Rust services run on the host, not in containers. |
| `infra/prod/` | Production-like Docker Compose stack — gateway, databases, and the three Rust services (`workout-app`, `integration-app`, `timeline-app`) all containerized. See `infra/prod/README.md` for setup, network model, and known limitations. |
| `infra/certs/`, `infra/dev/certs/` | TLS certs used by the gRPC services and the nginx gateway. |

Each of `workout/`, `timeline/`, `socialgym_web/`, `socialgym_mobile/`, and
`lapidation_mobile/` is a standalone project with its own dependency
manifest — there is no shared root build.

## Prerequisites

- Rust toolchain (`rustc`/`cargo`) — for `workout` and `timeline`
- Node.js + Yarn — for `socialgym_web`
- Flutter SDK — for `socialgym_mobile` and `lapidation_mobile`
- Docker + Docker Compose — for the infra stack (Postgres/Mongo/gateway)

## How to run it locally

The typical local setup runs the databases and gateway in Docker, and the
two Rust services directly on the host so you get fast rebuild loops.

### 1. Start infra dependencies (Postgres, Mongo, gateway)

```bash
cd infra/dev
cp .env.example .env   # fill in POSTGRES_PASSWORD, MONGO_*_PASSWORD, etc.
# TLS private key is gitignored - generate your own once:
openssl req -x509 -nodes -newkey rsa:2048 -days 825 \
  -keyout certs/server.key -out certs/server.crt \
  -config ../certs/san.cnf -extensions v3_req
docker compose up -d
```

This exposes Postgres on `5432`, Mongo on `27017`, and the nginx gateway on
`80`/`443` (proxying `/workout/api/`, `/timeline/api/`, `/login`, `/signup`,
`/auth/`, and gRPC traffic under `/grpc.` to the host-run services below).

### 2. Run `workout` (REST on :8090, gRPC on :50051)

```bash
cd workout
cp .env.example .env   # set DATABASE_URL to match infra/dev/.env, plus secrets/AWS config
cargo run
```

Migrations run automatically on boot.

### 3. Run `timeline` (REST on :8091)

```bash
cd timeline
# create .env with HOST, PORT, DATABASE_NAME, DATABASE_URL (Mongo), and
# ACCESS_TOKEN_SECRET/REFRESH_TOKEN_SECRET matching workout's, plus the
# GRPC_* vars pointing at workout's gRPC endpoint
cargo run
```

### 4. Run the web frontend

```bash
cd socialgym_web
yarn
yarn dev
```

### 5. Run a mobile app

```bash
cd socialgym_mobile   # or lapidation_mobile
flutter pub get
flutter run
```

Both mobile apps only target iOS and Android.

### Alternative: full containerized stack

`infra/prod/` runs everything — gateway, databases, and all three Rust
services — in Docker, closer to a real deployment. See
`infra/prod/README.md` for the setup steps (env vars, TLS certs, network
model). Use this if you want to test the whole system without running Rust
services on the host.

## Common commands

Rust services (run from inside `workout/` or `timeline/`, not the repo root):

```bash
cargo check
cargo build --release --package workout    # or --package timeline
cargo test
cargo test --features mock --test mock     # business-logic unit tests (workout)
```

Web frontend (`socialgym_web/`):

```bash
yarn dev
yarn build
yarn lint
```

Mobile apps regenerate gRPC stubs from `.proto` files with:

```bash
./tool/generate_proto.sh
```

Keep the mirrored `.proto` files in sync across `workout/integration/proto/`,
`timeline/business/proto/`, and each mobile app's `proto/` when changing the
gRPC contract.

See `CLAUDE.md` for the fuller architectural notes (crate layering, domain
rules, migration conventions) that guide day-to-day development in this
repo.
