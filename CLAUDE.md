# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Product domain

SocialGym is a social network plus features to monitor and keep information about wellness life. Read this section before the layout below — several parts of the codebase only make sense against it.

There are three actors: **Person**, **Business Profile — Professional**, and **Business Profile — Company**.

### Person (personal profile)

- Social: share photos, videos and posts; comment and react.
- Training: create workouts with exercises, execute them, keep progress information about the exercises.
- Sharing: share workouts and content with friends.
- Discovery: find professionals and companies.

### Business profile

Owned by a person (`business_profile.owner_id`). **One person may own more than one.** Two types, `ProfileType::{Professional, Company}` — note the type is `Company`, not "Business".

- **Professional** — a person offering wellness expertise: personal trainer, nutritionist, or any other legal wellness occupation. The same person can hold several (trainer *and* nutritionist). A Professional can have many persons, and **can take actions over the persons who granted permission**.
- **Company** — e.g. a gym. Can have many Professional business profiles and many persons, and **can take actions over both, where permission was granted**.

### The rule that governs the domain

**Authority is delegated by consent, never implied.** Every "can take actions over" edge above requires an explicit grant from the subject. Do not add a feature where a Professional or Company reaches a person's data by virtue of association alone — association and permission are separate facts.

### Built vs. intended

Much of the model above is not implemented yet. Do not write code that assumes it is.

| Domain concept | Status |
|---|---|
| Person; posts/comments/reactions; workouts, exercises, progress | Built |
| `business_profile` with `Professional`/`Company` type | Built — migration `m20260129_000009` |
| Person owns many business profiles | Table built (`profile`, `m20260714_000001`) but **unwired above the gateway layer** — no use case, no controller |
| Profile switching (re-issues a JWT with active-profile claims) | Built — `business/src/use_cases/switch_business_profile.rs` |
| Professional ↔ person relationship | **Not built** — `friends` is symmetric peer-to-peer only |
| Company ↔ Professional relationship | **Not built** — no business_profile ↔ business_profile link exists |
| Permission grants / acting on behalf of | **Not built** — nothing in either service models this |
| `Visibility::Professional` | Enum variant exists, but **no relation can resolve its audience** until grants land |
| Find professionals and companies (Marketplace) | **Not built** — a `header.marketplaceTitle` locale string and nothing else |

When the permission model is built, follow the consent pattern the codebase already has: `friends.status` (request → accept). Don't invent a second one.

## Repository layout

This is a polyglot monorepo (not itself a git repo — each subproject below is its own independent git repository) containing the SocialGym platform:

- **`workout/`** — Rust/Axum REST API + Tonic gRPC service. Owns Person, User, Friends, Workout/Exercise, and Business Profile data in PostgreSQL/PostGIS via SeaORM. This is the **source of truth for identity/social-graph data** — other services read it over gRPC rather than duplicating it.
- **`timeline/`** — Rust/Axum REST API. Owns Post/Feed, Reactions, Comments, Notifications, Evolution Check-ins in MongoDB. Calls into `workout`'s gRPC service (`integration` package) as a client for Person/Friend/User data — see `timeline/business/src/commons/grpc_config.rs` and the `.proto` files mirrored under `timeline/business/proto/`.
- **`socialgym_web/`** — React 19 + Vite + Redux Toolkit web frontend.
- **`socialgym_mobile/`** — Flutter mobile app, consumes both REST (via `dio`/`http`) and gRPC (via generated Dart stubs in `lib/src/generated/grpc/`) directly against `workout`'s `integration` gRPC service.
- **`billion/`** and **`development/`** — Docker Compose stacks (production-like and local dev respectively) wiring up nginx gateway, PostgreSQL/PostGIS, MongoDB, and the Rust services. `development/` is the one to use for local work.
- **`certs/`** — TLS certs shared by the gRPC services and nginx gateway.

## Rust services (`workout`, `timeline`)

Both are Cargo workspaces split into layered crates. Always build/test from inside the relevant service directory (`workout/` or `timeline/`), not the monorepo root — there is no top-level `Cargo.toml`.

### Layering (both services follow the same shape)

```
entity / domain   →  business            →  application          (+ integration, workout only)
(DB models /         (gateway traits,       (Axum HTTP controllers,   (Tonic gRPC server exposing
 plain structs)       use_cases, mapping     routes, DI/AppState,      the same use_cases to
                       from entity→domain)    OpenAPI/utoipa docs)      timeline & mobile clients)
```

- `entity` (workout only) — SeaORM entities, one file per table, generated/maintained by hand to match `migration/`.
- `domain` (timeline) / `business/src/domain` (workout) — plain Rust structs representing business objects, decoupled from the ORM/driver. `business_error.rs` defines the single `BusinessError` type used across all use cases (`Result<T, BusinessError>`).
- `business/src/gateway` — persistence-access structs (not traits) with `async fn` methods taking `&DbConn` (workout, SeaORM) or a Mongo `Database`/collection handle (timeline). No repository trait abstraction — call the gateway struct's associated functions directly.
- `business/src/use_cases` — orchestration layer; controllers and gRPC services call into these, never gateways directly.
- `application/src/http` + `application/src/routes` — Axum controllers and route builders; `application/src/lib.rs` wires the `Router`, CORS, Swagger UI (`utoipa`), auth middleware, and runs migrations on boot (workout: `Migrator::up`).
- `integration/` (workout only) — Tonic gRPC server. Proto files live in `integration/proto/`; the same `.proto` files are mirrored (and must stay in sync) under `timeline/business/proto/` and `socialgym_mobile/proto/` for their respective clients.
- `migration/` (workout only) — SeaORM migrations, filenames timestamp-prefixed (`m20260129_...`). Timeline has no migration crate since MongoDB is schemaless.

### Common commands

Run from inside `workout/` or `timeline/`:

```bash
cargo check                          # fast type-check across the workspace
cargo build --release --package workout   # (or --package timeline) — what the Dockerfile builds
cargo test                           # runs default tests
cargo test --features mock --test mock    # business-logic unit tests against sea_orm::MockDatabase (workout)
cargo run                            # runs src/main.rs, which just calls application::main()
```

- Business-logic tests use the `mock` Cargo feature (see `business/Cargo.toml` `[features] mock = [...]`) with `sea_orm::MockDatabase` — they live in `business/tests/mock.rs` and `application/tests/mock.rs` and require `--features mock`.
- Both services load config via `dotenvy` from a `.env` file in the service root (`DATABASE_URL`, `HOST`, `PORT`, AWS creds, `ACCESS_TOKEN_SECRET`/`REFRESH_TOKEN_SECRET`, gRPC TLS settings, etc.) — never commit real secrets to these files.
- workout's `sea-schema` dependency is pinned to `=0.18.0`; do not bump it without checking the comment in `workout/Cargo.toml` about the `sea-orm` 2.0.0-rc.41 shim `?Send`/`Send` conflict.

## Frontend (`socialgym_web`)

```bash
cd socialgym_web
yarn dev        # vite dev server
yarn build
yarn lint
```

React function components under `src/pages/<Feature>/`, Redux Toolkit state in `src/redux/`, REST calls in `src/service/` via the shared axios instance in `src/axios.config.js`. i18n via `react-i18next`, locale files in `src/locales/<lang>/`.

## Mobile (`socialgym_mobile`)

Flutter app using `provider` for state (`lib/providers/`), `lib/services/` for REST (`dio`/`http`) and gRPC (`lib/services/grpc/`) API access, `lib/models/` for DTOs, `lib/pages/<feature>/` for screens.

gRPC stubs are generated, not hand-written:

```bash
./tool/generate_proto.sh   # regenerates lib/src/generated/grpc/ from proto/*.proto
```

Keep `.proto` files in `socialgym_mobile/proto/`, `workout/integration/proto/`, and `timeline/business/proto/` in sync when changing the gRPC contract.

## Local environment

`development/compose.yml` (dev) and `billion/compose.yml` (prod-like) both bring up: nginx `gateway`, PostgreSQL/PostGIS (workout's DB), MongoDB (timeline's DB), and the Rust app containers. TLS certs come from `certs/` (prod) or `development/certs/` (dev). The gRPC `integration-app` container serves on port 50051 and requires `../certs/server.crt`/`server.key`.
