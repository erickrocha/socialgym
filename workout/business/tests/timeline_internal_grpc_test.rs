//! TC-009: workout's gateway against the real `timeline` container (TLS, shared secret, proto).
//! Needs the infra/test stack: `infra/test/start.sh`, then `source infra/test/timeline-test-env.sh`.
use business::gateway::timeline_deletion_gateway::TimelineDeletionGateway;

#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn the_gateway_exports_and_deletes_through_the_real_timeline_over_tls() {
    let ca = std::env::var("GRPC_CERT_PATH").expect("source infra/test/timeline-test-env.sh");
    unsafe {
        std::env::set_var("TIMELINE_GRPC_URL", "https://localhost:18092");
        std::env::set_var("TIMELINE_GRPC_CERT_PATH", ca);
        std::env::set_var("TIMELINE_GRPC_DOMAIN", "localhost");
        std::env::set_var("INTERNAL_SERVICE_SECRET", "c005-internal-secret");
    }
    let person = "00000000-0000-0000-0000-00000000c009";
    let export = TimelineDeletionGateway::export_person_data(person)
        .await
        .expect("export");
    for key in ["posts", "evolutions", "workoutSessions", "notifications"] {
        assert!(
            export.get(key).is_some_and(|v| v.is_array()),
            "missing {key}"
        );
    }
    TimelineDeletionGateway::delete_person_data(person)
        .await
        .expect("delete");

    unsafe { std::env::set_var("INTERNAL_SERVICE_SECRET", "wrong-secret") };
    assert!(
        TimelineDeletionGateway::export_person_data(person)
            .await
            .is_err(),
        "a wrong secret is refused"
    );
}
