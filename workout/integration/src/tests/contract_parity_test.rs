//! C-010 task 10: the contract tests (TC-013, TC-016, TC-017) against the running `infra/test` stack, black box:
//! REST on `localhost:18090`, gRPC (TLS) on `localhost:18501`. Start it with `infra/test/start.sh`, then
//! `source infra/test/timeline-test-env.sh` (it exports `GRPC_CERT_PATH`) and run
//! `cargo test -p integration --bin integration contract_parity -- --include-ignored --test-threads=1`.
//!
//! The authorization sweeps are generated from the sources of truth, so a new operation is covered without
//! touching this file: every `rpc` of `proto/*.proto` and every row of `rest_inventory.csv`.
use reqwest::{Certificate, Client, Method, StatusCode};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const REST: &str = "http://localhost:18090";
const PASSWORD: &str = "Str0ng!Password";
const GRPC: &str = "https://localhost:18501";

/// Reachable without an access token on purpose (sign-up, sign-in, refresh, public legal text).
const PUBLIC_RPCS: &[&str] = &[
    "/grpc.auth.AuthService/Signup",
    "/grpc.auth.AuthService/Login",
    "/grpc.auth.AuthService/Refresh",
    "/grpc.legal.LegalDocumentService/ListLegalDocuments",
    "/grpc.legal.LegalDocumentService/GetLegalDocument",
];
const PUBLIC_ROUTES: &[&str] = &["GET /", "POST /signup", "POST /login", "POST /refresh", "GET /legal/documents", "GET /legal/documents/{document}"];

fn manifest(path: &str) -> String {
    format!("{}/{path}", env!("CARGO_MANIFEST_DIR"))
}

fn rest() -> Client {
    Client::new()
}

/// An HTTP/2 client that trusts the stack's test CA; enough to call any gRPC method by path with raw bytes.
fn grpc_http() -> Client {
    let ca = std::env::var("GRPC_CERT_PATH").expect("source infra/test/timeline-test-env.sh");
    let pem = fs::read(ca).expect("read the test CA");
    Client::builder()
        .add_root_certificate(Certificate::from_pem(&pem).unwrap())
        .http2_prior_knowledge()
        .build()
        .unwrap()
}

/// Every `/package.Service/Method` declared by the proto files of this crate.
fn rpc_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(manifest("proto")).unwrap().flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "proto") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        let package = text.lines().find_map(|l| l.trim().strip_prefix("package ")).map(|p| p.trim_end_matches(';').trim().to_string());
        let (Some(package), true) = (package, text.contains("service ")) else { continue };
        let mut service = String::new();
        for line in text.lines().map(str::trim) {
            if let Some(rest) = line.strip_prefix("service ") {
                service = rest.split(|c: char| c == '{' || c.is_whitespace()).next().unwrap().to_string();
            } else if let Some(rest) = line.strip_prefix("rpc ") {
                let method = rest.split(|c: char| c == '(' || c.is_whitespace()).next().unwrap();
                paths.push(format!("/{package}.{service}/{method}"));
            }
        }
    }
    paths.sort();
    paths
}

/// `(method, route)` for every row of the REST inventory.
fn rest_routes() -> Vec<(String, String)> {
    let text = fs::read_to_string(manifest("../rest_inventory.csv")).unwrap();
    text.lines()
        .skip(1)
        .filter_map(|line| line.split(',').next())
        .filter_map(|route| route.split_once(' '))
        .map(|(method, route)| (method.to_string(), route.to_string()))
        .collect()
}

/// The `grpc-status` of a raw unary call (an empty message); the layers answer before decoding it.
async fn grpc_status(client: &Client, path: &str, authorization: Option<&str>) -> String {
    let mut request = client
        .post(format!("{GRPC}{path}"))
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .body(vec![0u8; 5]);
    if let Some(value) = authorization {
        request = request.header("authorization", value);
    }
    let response = request.send().await.unwrap_or_else(|e| panic!("{path}: {e}"));
    response.headers().get("grpc-status").and_then(|v| v.to_str().ok()).unwrap_or("none").to_string()
}

fn unique(label: &str) -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{label}-{nanos}@example.test")
}

/// Sign-up and sign-in share one limit per address (20 a minute) and the suite makes more than that, so each
/// sign-up waits its turn; the limit stays as it is in the product.
async fn pace_auth_calls() {
    static CALLS: std::sync::Mutex<Vec<std::time::Instant>> = std::sync::Mutex::new(Vec::new());
    loop {
        let wait = {
            let mut calls = CALLS.lock().unwrap();
            let now = std::time::Instant::now();
            calls.retain(|at| now.duration_since(*at) < std::time::Duration::from_secs(61));
            if calls.len() < 18 {
                calls.push(now);
                None
            } else {
                Some(std::time::Duration::from_secs(61) - now.duration_since(calls[0]))
            }
        };
        match wait {
            None => return,
            Some(wait) => tokio::time::sleep(wait).await,
        }
    }
}

/// A new person through REST sign-up; returns the access token.
async fn sign_up(label: &str) -> String {
    sign_up_with_email(label).await.0
}

/// As [`sign_up`], also returning the email the person signed up with.
async fn sign_up_with_email(label: &str) -> (String, String) {
    pace_auth_calls().await;
    let email = unique(label);
    let body = json!({
        "firstname": label, "surname": "Contract", "dateOfBirth": "1990-01-01", "gender": "X",
        "email": email, "password": PASSWORD,
        "termsVersion": "1.0.0", "privacyVersion": "1.0.0", "termsAccepted": true, "privacyAccepted": true
    });
    let response = rest().post(format!("{REST}/signup")).json(&body).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK, "sign-up");
    let tokens: Value = response.json().await.unwrap();
    (tokens["accessToken"].as_str().expect("access token").to_string(), email)
}

/// A token that parses but whose signature no longer matches.
fn tampered(token: &str) -> String {
    let mut bytes = token.as_bytes().to_vec();
    let last = bytes.len() - 2;
    bytes[last] = if bytes[last] == b'A' { b'B' } else { b'A' };
    String::from_utf8(bytes).unwrap()
}

fn concrete(route: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in route.chars() {
        match c {
            '{' => {
                inside = true;
                out.push('1');
            }
            '}' => inside = false,
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out
}

/// TC-013 step 1 and TC-017 step 1, gRPC side: no credentials, a malformed header and a tampered token are
/// `UNAUTHENTICATED` for every method outside the allow-list, including the two internal ones.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn every_grpc_method_refuses_missing_malformed_and_tampered_credentials() {
    let client = grpc_http();
    let token = sign_up("tampered").await;
    let broken = tampered(&token);
    let paths = rpc_paths();
    assert!(paths.len() > 60, "the proto sweep found only {} methods", paths.len());
    let mut wrong = Vec::new();
    for path in paths.iter().filter(|p| !PUBLIC_RPCS.contains(&p.as_str())) {
        for (label, header) in [("none", None), ("malformed", Some("Token abc".to_string())), ("tampered", Some(format!("Bearer {broken}")))] {
            let status = grpc_status(&client, path, header.as_deref()).await;
            if status != "16" {
                wrong.push(format!("{path} [{label}] -> grpc-status {status}"));
            }
        }
    }
    assert!(wrong.is_empty(), "methods that answer without valid credentials:\n{}", wrong.join("\n"));
}

/// The same sweep over REST for every inventory row outside the allow-list: a tampered token is `401`.
/// A missing or malformed `Authorization` header is `401` too, as gRPC answers `UNAUTHENTICATED` (design.md W19).
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn every_rest_route_refuses_missing_and_tampered_credentials() {
    let client = rest();
    let token = sign_up("tampered-rest").await;
    let broken = tampered(&token);
    let routes = rest_routes();
    assert!(routes.len() > 90, "the inventory sweep found only {} routes", routes.len());
    let skipped: BTreeSet<&str> = PUBLIC_ROUTES.iter().copied().collect();
    let mut wrong = Vec::new();
    for (method, route) in routes.iter().filter(|(m, r)| !skipped.contains(format!("{m} {r}").as_str())) {
        // Swagger and the OpenAPI document are documentation, not data.
        if route.starts_with("/swagger") || route.starts_with("/api-docs") {
            continue;
        }
        let method = Method::from_bytes(method.as_bytes()).unwrap();
        for (label, header, expected) in [
            ("none", None, StatusCode::UNAUTHORIZED),
            ("malformed", Some("Token abc".to_string()), StatusCode::UNAUTHORIZED),
            ("tampered", Some(format!("Bearer {broken}")), StatusCode::UNAUTHORIZED),
        ] {
            let mut request = client.request(method.clone(), format!("{REST}{}", concrete(route)));
            if let Some(value) = &header {
                request = request.header("authorization", value);
            }
            let status = request.send().await.unwrap().status();
            if status != expected {
                wrong.push(format!("{method} {route} [{label}] -> {status}"));
            }
        }
    }
    assert!(wrong.is_empty(), "routes that answer without valid credentials:\n{}", wrong.join("\n"));
}

// ---- TC-016 / TC-017: the same operation over both transports -----------------------------------------

use crate::proto::exercise::exercise_service_client::ExerciseServiceClient;
use crate::proto::exercise::{exercise_request, Exercise, ExerciseRequest};
use crate::proto::friend::friend_service_client::FriendServiceClient;
use crate::proto::friend::{FriendProfileRequest, FriendRequestRequest};
use crate::proto::person::person_service_client::PersonServiceClient;
use crate::proto::person::{person_id_request, PersonIdRequest, SearchMentionableFriendsRequest};
use crate::proto::settings::settings_service_client::SettingsServiceClient;
use crate::proto::settings::SettingOwnerIdRequest;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::{Code, Request};

async fn channel() -> Channel {
    let ca = fs::read(std::env::var("GRPC_CERT_PATH").expect("source infra/test/timeline-test-env.sh")).unwrap();
    let tls = ClientTlsConfig::new().ca_certificate(tonic::transport::Certificate::from_pem(ca)).domain_name("localhost");
    Channel::from_static(GRPC).tls_config(tls).unwrap().connect().await.unwrap()
}

fn authed<T>(message: T, token: &str) -> Request<T> {
    let mut request = Request::new(message);
    request.metadata_mut().insert("authorization", format!("Bearer {token}").parse().unwrap());
    request
}

/// The gRPC code the REST status maps to.
fn code_of(status: u16) -> Code {
    match status {
        200..=299 => Code::Ok,
        400 | 422 => Code::InvalidArgument,
        401 => Code::Unauthenticated,
        403 => Code::PermissionDenied,
        404 => Code::NotFound,
        409 => Code::AlreadyExists,
        423 => Code::FailedPrecondition,
        429 => Code::ResourceExhausted,
        _ => Code::Unknown,
    }
}

async fn call(method: Method, path: &str, token: &str, body: Option<Value>) -> (u16, Value) {
    let mut request = rest().request(method, format!("{REST}{path}")).header("authorization", format!("Bearer {token}"));
    if let Some(body) = body {
        request = request.json(&body);
    }
    let response = request.send().await.unwrap();
    let status = response.status().as_u16();
    (status, response.json().await.unwrap_or(Value::Null))
}

struct Actor {
    token: String,
    id: i32,
    uuid: String,
    info_id: i64,
    email: String,
}

async fn actor(label: &str) -> Actor {
    let (token, email) = sign_up_with_email(label).await;
    let (status, me) = call(Method::GET, "/workout/api/people/me", &token, None).await;
    assert_eq!(status, 200, "GET /people/me");
    Actor {
        id: me["id"].as_i64().unwrap() as i32,
        uuid: me["uuid"].as_str().unwrap().to_string(),
        info_id: me["personInfo"]["id"].as_i64().unwrap_or(0),
        email,
        token,
    }
}

/// Records a disagreement between the REST status and the gRPC outcome, and returns the common code.
fn agree<T>(label: &str, rest: u16, grpc: &Result<tonic::Response<T>, tonic::Status>, wrong: &mut Vec<String>) -> Code {
    let grpc_code = grpc.as_ref().map(|_| Code::Ok).unwrap_or_else(|s| s.code());
    if code_of(rest) != grpc_code {
        wrong.push(format!("{label}: REST {rest} but gRPC {grpc_code:?}"));
    }
    grpc_code
}

/// TC-016 and TC-017 step 2/4 for the exercise forms: owner, stranger, by id and by uuid, over both
/// transports; the stranger gets `404` on mutations (W4) and nothing of the owner's changes.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn exercise_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (alice, carol) = (actor("alice-ex").await, actor("carol-ex").await);
    let mut grpc = ExerciseServiceClient::new(channel().await);
    let mut wrong = Vec::new();

    let created = json!({
        "name": "Contract squat", "ownerId": alice.id, "ownerName": "Alice", "ownerUuid": alice.uuid,
        "description": "d", "sets": 3, "category": "Force", "repsOrDuration": 10, "visibility": "Private"
    });
    let (status, exercise) = call(Method::POST, "/workout/api/exercises", &alice.token, Some(created)).await;
    assert_eq!(status, 201, "create over REST: {exercise}");
    let (id, uuid) = (exercise["id"].as_i64().unwrap() as i32, exercise["uuid"].as_str().unwrap().to_string());

    let by_id = || ExerciseRequest { identifier: Some(exercise_request::Identifier::Id(id)) };
    let by_uuid = || ExerciseRequest { identifier: Some(exercise_request::Identifier::Uuid(uuid.clone())) };

    for (who, name, owner) in [(&alice, "owner", true), (&carol, "stranger", false)] {
        let (r, body) = call(Method::GET, &format!("/workout/api/exercises/id/{id}"), &who.token, None).await;
        let g = grpc.get_exercise(authed(by_id(), &who.token)).await;
        let code = agree(&format!("get by id as {name}"), r, &g, &mut wrong);
        let (r2, _) = call(Method::GET, &format!("/workout/api/exercises/uuid/{uuid}"), &who.token, None).await;
        let g2 = grpc.get_exercise(authed(by_uuid(), &who.token)).await;
        agree(&format!("get by uuid as {name}"), r2, &g2, &mut wrong);
        if owner {
            assert_eq!(code, Code::Ok);
            let got = g.unwrap().into_inner();
            assert_eq!((got.name.as_str(), got.sets, got.category.as_str()), (body["name"].as_str().unwrap(), body["sets"].as_i64().unwrap() as i32, body["category"].as_str().unwrap()));
        } else {
            assert_ne!(code, Code::Ok, "a stranger never reads a private exercise");
        }
    }

    // Mutations by the stranger: equal, `404`, and nothing changes.
    let mut hijack = exercise.clone();
    hijack["name"] = json!("hijacked");
    let (r, _) = call(Method::PUT, "/workout/api/exercises", &carol.token, Some(hijack)).await;
    let proto = Exercise { id, uuid: uuid.clone(), name: "hijacked".into(), owner_id: alice.id, owner_uuid: alice.uuid.clone(), owner_name: "Alice".into(), description: "d".into(), sets: 3, category: "Force".into(), reps_or_duration: 10, visibility: "Private".into(), ..Default::default() };
    let g = grpc.update_exercise(authed(proto, &carol.token)).await;
    assert_eq!(agree("update as stranger", r, &g, &mut wrong), Code::NotFound);
    let (r, _) = call(Method::DELETE, &format!("/workout/api/exercises/uuid/{uuid}"), &carol.token, None).await;
    let g = grpc.delete_exercise(authed(by_uuid(), &carol.token)).await;
    assert_eq!(agree("delete by uuid as stranger", r, &g, &mut wrong), Code::NotFound);
    let (r, _) = call(Method::DELETE, &format!("/workout/api/exercises/{id}"), &carol.token, None).await;
    let g = grpc.delete_exercise(authed(by_id(), &carol.token)).await;
    assert_eq!(agree("delete by id as stranger", r, &g, &mut wrong), Code::NotFound);

    let (status, after) = call(Method::GET, &format!("/workout/api/exercises/id/{id}"), &alice.token, None).await;
    assert_eq!((status, after["name"].as_str()), (200, Some("Contract squat")), "the stranger changed nothing");
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

/// TC-016 and TC-017 for people, friends and settings: the friend profile hides weight and height (W18),
/// strangers are refused alike, and the owner checks of `SearchMentionableFriends` and the settings reads hold.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn person_friend_and_settings_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (alice, bob, carol) = (actor("alice-pf").await, actor("bob-pf").await, actor("carol-pf").await);
    let channel = channel().await;
    let (mut people, mut friends, mut settings) = (PersonServiceClient::new(channel.clone()), FriendServiceClient::new(channel.clone()), SettingsServiceClient::new(channel));
    let mut wrong = Vec::new();

    // Bob has a weight and a height; Alice and Bob become friends (request over REST, accept over gRPC).
    let consent = json!({"document": "health_data", "version": "1.0.0", "accepted": true});
    let (status, _) = call(Method::POST, "/workout/api/people/me/consents", &bob.token, Some(consent)).await;
    assert!(status < 300, "Bob accepts the health-data consent: {status}");
    let info = json!({"id": bob.info_id, "personId": bob.id, "weight": 80.0, "height": 180.0, "job": "coach"});
    let (status, _) = call(Method::PUT, &format!("/workout/api/people/me/info/{}", bob.info_id), &bob.token, Some(info)).await;
    assert_eq!(status, 200, "set Bob's info");
    let (status, _) = call(Method::PUT, &format!("/workout/api/friends/request/{}", bob.id), &alice.token, None).await;
    assert_eq!(status, 200, "Alice asks Bob");
    friends.accept_friend_request(authed(FriendRequestRequest { person_id: alice.id }, &bob.token)).await.expect("Bob accepts over gRPC");

    // The friend profile: Alice reads Bob's; weight and height are hidden on both.
    let (r, body) = call(Method::GET, &format!("/workout/api/people/me/friend/{}", bob.id), &alice.token, None).await;
    let g = friends.get_friend_profile(authed(FriendProfileRequest { friend_id: bob.id }, &alice.token)).await;
    assert_eq!(agree("friend profile as friend", r, &g, &mut wrong), Code::Ok);
    let info = g.unwrap().into_inner().person.unwrap().person_info.unwrap();
    assert!(body["personInfo"]["weight"].is_null() && body["personInfo"]["height"].is_null(), "REST hides weight and height: {body}");
    assert_eq!((info.weight, info.height, info.job.as_str()), (0.0, 0.0, "coach"), "gRPC hides them too, the rest stays");
    // The same profile as its owner reads it keeps them.
    let (_, own) = call(Method::GET, "/workout/api/people/me", &bob.token, None).await;
    assert_eq!(own["personInfo"]["weight"].as_f64(), Some(80.0));

    // A stranger and the caller's own id.
    let (r, _) = call(Method::GET, &format!("/workout/api/people/me/friend/{}", bob.id), &carol.token, None).await;
    let g = friends.get_friend_profile(authed(FriendProfileRequest { friend_id: bob.id }, &carol.token)).await;
    assert_eq!(agree("friend profile as stranger", r, &g, &mut wrong), Code::PermissionDenied);
    let (r, _) = call(Method::GET, &format!("/workout/api/people/me/friend/{}", alice.id), &alice.token, None).await;
    let g = friends.get_friend_profile(authed(FriendProfileRequest { friend_id: alice.id }, &alice.token)).await;
    assert_eq!(agree("friend profile of oneself", r, &g, &mut wrong), Code::InvalidArgument);

    // Another person's record by id, uuid and the mentionable-friends search (owner only).
    let by_id = |id| PersonIdRequest { identifier: Some(person_id_request::Identifier::Id(id)) };
    for (who, name) in [(&alice, "owner"), (&carol, "stranger")] {
        let (r, _) = call(Method::GET, &format!("/workout/api/people/id/{}", alice.id), &who.token, None).await;
        let g = people.get_person(authed(by_id(alice.id), &who.token)).await;
        let code = agree(&format!("person by id as {name}"), r, &g, &mut wrong);
        assert_eq!(code == Code::Ok, name == "owner");
        let (r, _) = call(Method::GET, &format!("/workout/api/people/id/{}/mentionable-friends?query=a", alice.id), &who.token, None).await;
        let g = people.search_mentionable_friends(authed(SearchMentionableFriendsRequest { person_id: alice.id, query: "a".into(), limit: 5 }, &who.token)).await;
        let code = agree(&format!("mentionable friends as {name}"), r, &g, &mut wrong);
        assert_eq!(code == Code::Ok, name == "owner");
        let (r, _) = call(Method::GET, &format!("/workout/api/settings/owner/id/{}", alice.id), &who.token, None).await;
        let g = settings.get_by_owner_ids(authed(SettingOwnerIdRequest { owner_id: alice.id, owner_uuid: String::new() }, &who.token)).await;
        agree(&format!("settings by owner id as {name}"), r, &g, &mut wrong);
    }
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

use crate::proto::business_profile::business_profile_service_client::BusinessProfileServiceClient;
use crate::proto::business_profile::{BusinessProfile, BusinessProfileRequestId, DeleteBusinessProfileRequest};
use crate::proto::consent::consent_service_client::ConsentServiceClient;
use crate::proto::consent::{ListConsentsRequest, ListPendingConsentsRequest};
use crate::proto::legal::legal_document_service_client::LegalDocumentServiceClient;
use crate::proto::legal::ListLegalDocumentsRequest;

/// TC-016 for the consent and legal-document reads (equal contents) and TC-017 for the business-profile
/// mutations: a stranger is refused alike on both transports (`403`, kept by the owner decision) and the
/// street-level address of a profile is hidden from a stranger on both.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn consent_legal_and_business_profile_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (alice, carol) = (actor("alice-bp").await, actor("carol-bp").await);
    let channel = channel().await;
    let (mut consents, mut legal, mut profiles) = (ConsentServiceClient::new(channel.clone()), LegalDocumentServiceClient::new(channel.clone()), BusinessProfileServiceClient::new(channel));
    let mut wrong = Vec::new();

    // Consents: the same documents and versions on both transports.
    let (_, rest_list) = call(Method::GET, "/workout/api/people/me/consents", &alice.token, None).await;
    let grpc_list = consents.list_consents(authed(ListConsentsRequest {}, &alice.token)).await.unwrap().into_inner();
    let rest_docs: BTreeSet<String> = rest_list.as_array().unwrap().iter().map(|c| format!("{}@{}", c["document"].as_str().unwrap(), c["version"].as_str().unwrap())).collect();
    let grpc_docs: BTreeSet<String> = grpc_list.consents.iter().map(|c| format!("{}@{}", c.document, c.version)).collect();
    assert_eq!(rest_docs, grpc_docs, "consents");
    assert!(rest_docs.len() >= 2, "terms and privacy were accepted at sign-up");
    let (_, rest_pending) = call(Method::GET, "/workout/api/people/me/consents/pending", &alice.token, None).await;
    let grpc_pending = consents.list_pending_consents(authed(ListPendingConsentsRequest {}, &alice.token)).await.unwrap().into_inner();
    assert_eq!(rest_pending.as_array().map(Vec::len), Some(grpc_pending.pending.len()), "pending consents");

    // Legal documents (public): the same set.
    let (status, rest_legal) = call(Method::GET, "/legal/documents", "", None).await;
    assert_eq!(status, 200);
    let grpc_legal = legal.list_legal_documents(Request::new(ListLegalDocumentsRequest {})).await.unwrap().into_inner();
    assert_eq!(rest_legal.as_array().map(Vec::len), Some(grpc_legal.documents.len()), "legal documents");

    // Alice owns a profile with an address; she creates it over REST.
    let profile = json!({"ownerId": alice.id, "ownerUuid": alice.uuid, "taxId": "12345678000199", "businessName": "Contract Gym", "businessType": "Company", "addresses": []});
    let (status, created) = call(Method::POST, "/workout/api/business-profiles", &alice.token, Some(profile)).await;
    assert_eq!(status, 201, "create the profile: {created}");
    let (id, uuid) = (created["id"].as_i64().unwrap() as i32, created["uuid"].as_str().unwrap().to_string());

    // Reads: the owner and a stranger agree across transports; the stranger does not see the owner's ids.
    for (who, name) in [(&alice, "owner"), (&carol, "stranger")] {
        let (r, body) = call(Method::GET, &format!("/workout/api/business-profiles/id/{id}"), &who.token, None).await;
        let g = profiles.get_business_profile_by_id(authed(BusinessProfileRequestId { id, uuid: String::new() }, &who.token)).await;
        agree(&format!("profile by id as {name}"), r, &g, &mut wrong);
        if name == "stranger" && r == 200 {
            assert!(body["ownerId"].as_i64().unwrap_or(0) == 0 && body["ownerUuid"].as_str().unwrap_or("").is_empty(), "REST hides the owner ids: {body}");
            let grpc = g.unwrap().into_inner();
            assert!(grpc.owner_id == 0 && grpc.owner_uuid.is_empty(), "gRPC hides the owner ids");
        }
    }

    // Mutations by the stranger: equal and `403`; the profile is unchanged.
    let mut changed = created.clone();
    changed["businessName"] = json!("Stolen");
    let (r, _) = call(Method::PUT, "/workout/api/business-profiles", &carol.token, Some(changed)).await;
    let g = profiles.update_business_profile(authed(BusinessProfile { id, uuid: uuid.clone(), owner_id: alice.id, owner_uuid: alice.uuid.clone(), tax_id: "12345678000199".into(), business_name: "Stolen".into(), business_type: "Company".into(), ..Default::default() }, &carol.token)).await;
    assert_eq!(agree("update profile as stranger", r, &g, &mut wrong), Code::PermissionDenied);
    let (r, _) = call(Method::DELETE, &format!("/workout/api/business-profiles/id/{id}"), &carol.token, None).await;
    let g = profiles.delete_business_profile(authed(DeleteBusinessProfileRequest { id }, &carol.token)).await;
    assert_eq!(agree("delete profile as stranger", r, &g, &mut wrong), Code::PermissionDenied);

    let (status, after) = call(Method::GET, &format!("/workout/api/business-profiles/id/{id}"), &alice.token, None).await;
    assert_eq!((status, after["businessName"].as_str()), (200, Some("Contract Gym")), "the stranger changed nothing");
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

/// TC-016: the pre-signed upload URL of the Active Business Profile's logo and cover, over both transports,
/// and the refusal when no profile is active.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn business_profile_image_upload_urls_give_equal_outcomes_over_rest_and_grpc() {
    use crate::proto::business_profile::BusinessProfileImageUploadRequest;
    let alice = actor("alice-img").await;
    let mut profiles = BusinessProfileServiceClient::new(channel().await);
    let mut wrong = Vec::new();

    // Created over gRPC the way the app does: the image fields are empty strings, not absent.
    let made = profiles
        .add_business_profile(authed(BusinessProfile { owner_id: alice.id, owner_uuid: alice.uuid.clone(), tax_id: "12345678000199".into(), business_name: "Image Gym".into(), business_type: "Company".into(), ..Default::default() }, &alice.token))
        .await
        .expect("create over gRPC")
        .into_inner();
    let created = json!({"uuid": made.uuid});

    // No active profile: the same refusal on both.
    let (r, _) = call(Method::GET, "/workout/api/business-profiles/upload/avatar?format=image/png", &alice.token, None).await;
    let g = profiles.get_business_profile_image_upload_url(authed(BusinessProfileImageUploadRequest { image_type: "logo".into(), format: "image/png".into() }, &alice.token)).await;
    // TC-007 step 3: REST answers 400 (it was 500) and gRPC FAILED_PRECONDITION, both without a URL.
    assert_eq!(r, 400, "REST: no active profile is a client error");
    assert_eq!(g.as_ref().map(|_| ()).map_err(|s| s.code()), Err(Code::FailedPrecondition), "gRPC: the same condition");

    // Active profile: switch over REST, then ask on both.
    let uuid = created["uuid"].as_str().unwrap();
    let (status, active) = call(Method::POST, &format!("/auth/profile/{uuid}/activate"), &alice.token, None).await;
    assert_eq!(status, 200, "{active}");
    let token = active["accessToken"].as_str().unwrap().to_string();
    for (rest_type, grpc_type) in [("avatar", "logo"), ("cover", "cover")] {
        let (r, body) = call(Method::GET, &format!("/workout/api/business-profiles/upload/{rest_type}?format=image/png"), &token, None).await;
        let g = profiles.get_business_profile_image_upload_url(authed(BusinessProfileImageUploadRequest { image_type: grpc_type.into(), format: "image/png".into() }, &token)).await;
        println!("{grpc_type}: REST {r} {body}, gRPC {:?}", g.as_ref().map(|x| x.get_ref().object_key.clone()).map_err(|s| (s.code(), s.message().to_string())));
        agree(&format!("{grpc_type} upload URL"), r, &g, &mut wrong);
        assert_eq!(r, 200, "REST gives the URL: {body}");
    }
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

/// TC-013 steps 4 and 5: a message over the size limit is refused before any use case runs
/// (`OUT_OF_RANGE`, 11), and a stopped database is `UNAVAILABLE` (14), not a refusal of the caller.
#[tokio::test]
#[ignore = "requires the infra/test stack; stops and restarts its PostgreSQL"]
async fn an_oversized_message_and_a_stopped_database_have_their_own_statuses() {
    let alice = actor("alice-limits").await;
    let client = grpc_http();
    let bearer = format!("Bearer {}", alice.token);

    // 6 MiB declared and sent (the server's limit is 5 MiB), to a method that takes a message.
    let mut frame = vec![0u8];
    frame.extend_from_slice(&(6u32 * 1024 * 1024).to_be_bytes());
    frame.extend(vec![0u8; 6 * 1024 * 1024]);
    let response = client
        .post(format!("{GRPC}/grpc.person.PersonService/updatePerson"))
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .header("authorization", &bearer)
        .body(frame)
        .send()
        .await
        .unwrap();
    let status = response.headers().get("grpc-status").and_then(|v| v.to_str().ok()).unwrap_or("none").to_string();
    assert_eq!(status, "11", "OUT_OF_RANGE for a message over the limit");

    let docker = |action: &str| {
        let ok = std::process::Command::new("docker").args([action, "test-postgres-1"]).status().unwrap().success();
        assert!(ok, "docker {action} test-postgres-1");
    };
    docker("stop");
    let during = grpc_status(&client, "/grpc.person.PersonService/GetMe", Some(&bearer)).await;
    docker("start");
    for _ in 0..60 {
        let health = std::process::Command::new("docker").args(["inspect", "-f", "{{.State.Health.Status}}", "test-postgres-1"]).output().unwrap();
        if String::from_utf8_lossy(&health.stdout).trim() == "healthy" {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    assert_eq!(during, "14", "UNAVAILABLE while the database is down");
}

use crate::proto::account::account_service_client::AccountServiceClient;
use crate::proto::account::{CancelAccountDeletionRequest, CreateDataExportRequest, GetDataExportRequest, ListDataExportsRequest, RequestAccountDeletionRequest};
use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{ActivateBusinessProfileRequest, DeactivateBusinessProfileRequest, LogoutRequest};
use crate::proto::team_member::team_member_service_client::TeamMemberServiceClient;
use crate::proto::team_member::{TeamMemberPageRequest, TeamMemberRequest};
use crate::proto::workout::workout_service_client::WorkoutServiceClient;
use crate::proto::workout::{workout_request, WorkoutExercisesRequest, WorkoutRequest};

/// TC-016 and TC-017 for workouts: the owner and a stranger read, compose and delete a workout by id and by
/// uuid; the stranger is told it does not exist on both transports and changes nothing.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn workout_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (alice, carol) = (actor("alice-wk").await, actor("carol-wk").await);
    let mut grpc = WorkoutServiceClient::new(channel().await);
    let mut wrong = Vec::new();
    let body = json!({"ownerId": alice.id, "ownerUuid": alice.uuid, "name": "Contract legs", "difficulty": "Soft", "muscleGroup": "legs", "visibility": "Private"});
    let (s1, first) = call(Method::POST, "/workout/api/workouts", &alice.token, Some(body.clone())).await;
    let (s2, second) = call(Method::POST, "/workout/api/workouts", &alice.token, Some(body)).await;
    assert!(s1 < 300 && s2 < 300, "{first} {second}");
    let (id, uuid) = (first["id"].as_i64().unwrap() as i32, first["uuid"].as_str().unwrap().to_string());
    let by_id = |id| WorkoutRequest { identifier: Some(workout_request::Identifier::Id(id)) };

    for (who, name) in [(&alice, "owner"), (&carol, "stranger")] {
        let (r, rest) = call(Method::GET, &format!("/workout/api/workouts/id/{id}"), &who.token, None).await;
        let g = grpc.get_workout(authed(by_id(id), &who.token)).await;
        let code = agree(&format!("get by id as {name}"), r, &g, &mut wrong);
        let (r, _) = call(Method::GET, &format!("/workout/api/workouts/uuid/{uuid}"), &who.token, None).await;
        let g2 = grpc.get_workout(authed(WorkoutRequest { identifier: Some(workout_request::Identifier::Uuid(uuid.clone())) }, &who.token)).await;
        agree(&format!("get by uuid as {name}"), r, &g2, &mut wrong);
        let (r, _) = call(Method::GET, &format!("/workout/api/workouts/{id}/exercises"), &who.token, None).await;
        let g3 = grpc.get_workout_exercises(authed(by_id(id), &who.token)).await;
        agree(&format!("exercises of a workout as {name}"), r, &g3, &mut wrong);
        if name == "owner" {
            assert_eq!(code, Code::Ok);
            assert_eq!(g.unwrap().into_inner().name, rest["name"].as_str().unwrap());
        } else {
            assert_ne!(code, Code::Ok, "a stranger never reads a private workout");
        }
    }

    // Composing exercises: the owner may over both forms, a stranger is told there is no such workout.
    let exercise = json!([{"name": "Squat", "ownerId": alice.id, "ownerName": "Alice", "ownerUuid": alice.uuid, "category": "Force", "sets": 3, "repsOrDuration": 10, "visibility": "Private"}]);
    let proto_exercise = Exercise { name: "Squat".into(), owner_id: alice.id, owner_uuid: alice.uuid.clone(), owner_name: "Alice".into(), category: "Force".into(), sets: 3, reps_or_duration: 10, visibility: "Private".into(), ..Default::default() };
    for (who, name) in [(&alice, "owner"), (&carol, "stranger")] {
        let (r, _) = call(Method::POST, &format!("/workout/api/workouts/{id}/exercises"), &who.token, Some(exercise.clone())).await;
        let g = grpc.add_exercises_to_workout(authed(WorkoutExercisesRequest { workout_uuid: String::new(), exercises: vec![proto_exercise.clone()], workout_id: id }, &who.token)).await;
        let code = agree(&format!("compose by id as {name}"), r, &g, &mut wrong);
        assert_eq!(code == Code::Ok, name == "owner");
    }

    // Deleting: the stranger gets `404` and nothing changes; the owner deletes one workout over each transport.
    let (r, _) = call(Method::DELETE, &format!("/workout/api/workouts/id/{id}"), &carol.token, None).await;
    let g = grpc.delete_workout(authed(by_id(id), &carol.token)).await;
    assert_eq!(agree("delete as stranger", r, &g, &mut wrong), Code::NotFound);
    let (r, _) = call(Method::DELETE, &format!("/workout/api/workouts/id/{id}"), &alice.token, None).await;
    assert!(r < 300, "REST delete by the owner");
    let second_id = second["id"].as_i64().unwrap() as i32;
    grpc.delete_workout(authed(by_id(second_id), &alice.token)).await.expect("gRPC delete by the owner");
    let (r, _) = call(Method::GET, &format!("/workout/api/workouts/id/{second_id}"), &alice.token, None).await;
    assert_eq!(r, 404, "the workout the gRPC call deleted is gone for REST too");
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

/// TC-003 and TC-016 for the session and the account: profile switch (owner and stranger), logout, account
/// deletion requested and cancelled, and data exports (create, list, read, download before it is ready,
/// another person's export), each done once over REST and once over gRPC.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn session_account_and_export_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (alice, carol, rest_person, grpc_person) = (actor("alice-ss").await, actor("carol-ss").await, actor("rest-ss").await, actor("grpc-ss").await);
    let channel = channel().await;
    let (mut auth, mut account) = (AuthServiceClient::new(channel.clone()), AccountServiceClient::new(channel));
    let mut wrong = Vec::new();

    // Profile switch. Switching revokes the token it was made with, so each transport has its own owner.
    let make_profile = |who: &Actor| json!({"ownerId": who.id, "ownerUuid": who.uuid, "taxId": "12345678000199", "businessName": "Switch Gym", "businessType": "Company", "addresses": []});
    let grpc_owner = actor("grpc-owner-ss").await;
    let (status, rest_profile) = call(Method::POST, "/workout/api/business-profiles", &alice.token, Some(make_profile(&alice))).await;
    assert_eq!(status, 201);
    let (status, grpc_profile) = call(Method::POST, "/workout/api/business-profiles", &grpc_owner.token, Some(make_profile(&grpc_owner))).await;
    assert_eq!(status, 201);
    let (rest_uuid, grpc_uuid) = (rest_profile["uuid"].as_str().unwrap().to_string(), grpc_profile["uuid"].as_str().unwrap().to_string());
    let (r, rest_active) = call(Method::POST, &format!("/auth/profile/{rest_uuid}/activate"), &alice.token, None).await;
    let g = auth.activate_business_profile(authed(ActivateBusinessProfileRequest { business_profile_uuid: grpc_uuid.clone() }, &grpc_owner.token)).await;
    assert_eq!(agree("activate as owner", r, &g, &mut wrong), Code::Ok);
    assert_eq!(rest_active["activeBusinessProfileUuid"].as_str(), Some(rest_uuid.as_str()));
    let grpc_token = g.unwrap().into_inner();
    assert_eq!(grpc_token.active_business_profile_uuid.as_deref(), Some(grpc_uuid.as_str()));
    let (r, _) = call(Method::POST, &format!("/auth/profile/{rest_uuid}/activate"), &carol.token, None).await;
    let g = auth.activate_business_profile(authed(ActivateBusinessProfileRequest { business_profile_uuid: grpc_uuid.clone() }, &carol.token)).await;
    assert_eq!(agree("activate as stranger", r, &g, &mut wrong), Code::PermissionDenied);
    let rest_token = rest_active["accessToken"].as_str().unwrap().to_string();
    let (r, rest_back) = call(Method::POST, "/auth/profile/deactivate", &rest_token, None).await;
    let g = auth.deactivate_business_profile(authed(DeactivateBusinessProfileRequest {}, &grpc_token.access_token)).await;
    assert_eq!(agree("deactivate", r, &g, &mut wrong), Code::Ok);
    let (rest_session, grpc_session) = (rest_back["accessToken"].as_str().unwrap().to_string(), g.unwrap().into_inner().access_token);

    // Data exports: the same steps, REST for one person and gRPC for another; Carol cannot see either.
    let (r, rest_export) = call(Method::POST, "/workout/api/people/me/data-exports", &rest_person.token, None).await;
    let g = account.create_data_export(authed(CreateDataExportRequest {}, &grpc_person.token)).await;
    assert_eq!(agree("create export", r, &g, &mut wrong), Code::Ok);
    let grpc_export = g.unwrap().into_inner();
    assert_eq!(rest_export["status"].as_str(), Some(grpc_export.status.as_str()), "the same initial status");
    let rest_id = rest_export["id"].as_str().unwrap().to_string();
    let (r, listed) = call(Method::GET, "/workout/api/people/me/data-exports", &rest_person.token, None).await;
    let g = account.list_data_exports(authed(ListDataExportsRequest {}, &grpc_person.token)).await;
    agree("list exports", r, &g, &mut wrong);
    assert_eq!(listed.as_array().map(Vec::len), Some(g.unwrap().into_inner().exports.len()));
    let (r, _) = call(Method::GET, &format!("/workout/api/people/me/data-exports/{rest_id}"), &rest_person.token, None).await;
    let g = account.get_data_export(authed(GetDataExportRequest { id: grpc_export.id.clone() }, &grpc_person.token)).await;
    agree("read an export", r, &g, &mut wrong);
    let (r, _) = call(Method::GET, &format!("/workout/api/people/me/data-exports/{rest_id}"), &carol.token, None).await;
    let g = account.get_data_export(authed(GetDataExportRequest { id: grpc_export.id.clone() }, &carol.token)).await;
    assert_eq!(agree("read another person's export", r, &g, &mut wrong), Code::NotFound);

    // Account deletion requested and cancelled; then logout, last because it ends the session.
    let (r, rest_deletion) = call(Method::POST, "/workout/api/people/me/account/delete", &rest_person.token, Some(json!({"immediate": false}))).await;
    let g = account.request_account_deletion(authed(RequestAccountDeletionRequest { immediate: false }, &grpc_person.token)).await;
    assert_eq!(agree("request deletion", r, &g, &mut wrong), Code::Ok);
    let grpc_status = g.unwrap().into_inner();
    assert!(rest_deletion["scheduledAt"].is_string() && !grpc_status.scheduled_at.is_empty());
    // The request ends every session of the person, so each signs in again (after the second the
    // revocation counts from) and cancels with the new token, as the app does.
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    pace_auth_calls().await;
    let rest_login: Value = rest().post(format!("{REST}/login")).form(&[("email", rest_person.email.as_str()), ("password", PASSWORD)]).send().await.unwrap().json().await.unwrap();
    pace_auth_calls().await;
    let grpc_login = auth.login(Request::new(crate::proto::auth::LoginRequest { email: grpc_person.email.clone(), password: PASSWORD.into() })).await.expect("gRPC login").into_inner();
    assert_eq!(rest_login["pendingAccountDeletion"].is_object(), grpc_login.pending_account_deletion.is_some(), "both sign-ins report the pending deletion");
    let (r, _) = call(Method::POST, "/workout/api/people/me/account/cancel-deletion", rest_login["accessToken"].as_str().unwrap(), None).await;
    let g = account.cancel_account_deletion(authed(CancelAccountDeletionRequest {}, &grpc_login.access_token)).await;
    assert_eq!(agree("cancel deletion", r, &g, &mut wrong), Code::Ok);
    let (r, _) = call(Method::POST, "/logout", &rest_session, Some(json!({}))).await;
    let g = auth.logout(authed(LogoutRequest { refresh_token: String::new() }, &grpc_session)).await;
    agree("logout", r, &g, &mut wrong);
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}

/// TC-016 for the team: the page of a person with no team, a request as a person with no active profile
/// (refused alike) and a request from an active profile (accepted alike), then the person's answer.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn team_member_operations_give_equal_outcomes_over_rest_and_grpc() {
    let (owner, rest_member, grpc_member, nobody) = (actor("owner-tm").await, actor("rest-tm").await, actor("grpc-tm").await, actor("nobody-tm").await);
    let mut team = TeamMemberServiceClient::new(channel().await);
    let mut wrong = Vec::new();

    let (r, _) = call(Method::GET, "/workout/api/team-members", &nobody.token, None).await;
    let g = team.get_team_member_page(authed(TeamMemberPageRequest { business_profile_id: 0, person_id: nobody.id }, &nobody.token)).await;
    agree("team page of a person with no team", r, &g, &mut wrong);

    // A person with no active profile cannot invite anyone.
    let (r, _) = call(Method::PUT, &format!("/workout/api/team-members/request/{}", rest_member.id), &nobody.token, None).await;
    let g = team.send_team_member_request(authed(TeamMemberRequest { business_profile_id: 999_999, person_id: rest_member.id }, &nobody.token)).await;
    agree("invite without an active profile", r, &g, &mut wrong);

    // The owner activates a profile and invites one person over each transport.
    let profile = json!({"ownerId": owner.id, "ownerUuid": owner.uuid, "taxId": "12345678000199", "businessName": "Team Gym", "businessType": "Company", "addresses": []});
    let (status, created) = call(Method::POST, "/workout/api/business-profiles", &owner.token, Some(profile)).await;
    assert_eq!(status, 201);
    let profile_id = created["id"].as_i64().unwrap() as i32;
    let (status, active) = call(Method::POST, &format!("/auth/profile/{}/activate", created["uuid"].as_str().unwrap()), &owner.token, None).await;
    assert_eq!(status, 200);
    let acting = Actor { token: active["accessToken"].as_str().unwrap().to_string(), id: owner.id, uuid: owner.uuid.clone(), info_id: 0, email: owner.email.clone() };
    let (r, _) = call(Method::PUT, &format!("/workout/api/team-members/request/{}", rest_member.id), &acting.token, None).await;
    let g = team.send_team_member_request(authed(TeamMemberRequest { business_profile_id: profile_id, person_id: grpc_member.id }, &acting.token)).await;
    assert_eq!(agree("invite from an active profile", r, &g, &mut wrong), Code::Ok);

    // Each invited person accepts over the other transport's twin; a stranger cannot accept for them.
    let (r, _) = call(Method::PUT, &format!("/workout/api/team-members/accept/{profile_id}"), &rest_member.token, None).await;
    let g = team.accept_team_member_request(authed(TeamMemberRequest { business_profile_id: profile_id, person_id: grpc_member.id }, &grpc_member.token)).await;
    assert_eq!(agree("accept an invitation", r, &g, &mut wrong), Code::Ok);
    let (r, _) = call(Method::PUT, &format!("/workout/api/team-members/accept/{profile_id}"), &nobody.token, None).await;
    let g = team.accept_team_member_request(authed(TeamMemberRequest { business_profile_id: profile_id, person_id: nobody.id }, &nobody.token)).await;
    agree("accept without an invitation", r, &g, &mut wrong);
    assert!(wrong.is_empty(), "REST and gRPC differ:\n{}", wrong.join("\n"));
}
