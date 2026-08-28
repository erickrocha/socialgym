use crate::http::json::consent_json::ConsentJson;
use crate::infrastructure::mapper::{Mapper, PersonMapper};
use business::gateway::consent_gateway::ConsentGateway;
use business::gateway::data_export_gateway::DataExportGateway;
use business::gateway::person_media_gateway::PersonMediaGateway;
use business::gateway::timeline_deletion_gateway::TimelineDeletionGateway;
use business::sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use business::use_cases::image_storage_use_case::ImageStorageUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use chrono::{Duration, Utc};
use serde_json::json;
use std::io::{Cursor, Write};
use std::sync::Arc;
use tokio::time::{interval, Duration as TokioDuration};
use zip::write::SimpleFileOptions;

#[derive(FromQueryResult)]
struct ExportSnapshot {
    data: serde_json::Value,
}

async fn workout_snapshot(
    db: &DatabaseConnection,
    person_id: i32,
) -> Result<serde_json::Value, String> {
    // Keep this query explicit: it is both the export inventory and a safeguard
    // against accidentally including password hashes or token-revocation data.
    let sql = r#"
        SELECT jsonb_build_object(
          'identity', (SELECT to_jsonb(u) FROM (
            SELECT uuid, name, email, enabled, first_login, created_at, updated_at,
                   deletion_requested_at, deletion_scheduled_at
            FROM "user" WHERE person_id = $1
          ) u),
          'settings', COALESCE((SELECT jsonb_agg(to_jsonb(s)) FROM settings s WHERE person_id = $1), '[]'::jsonb),
          'friends', COALESCE((SELECT jsonb_agg(to_jsonb(f)) FROM friends f WHERE person_id = $1 OR friend_id = $1), '[]'::jsonb),
          'profiles', COALESCE((SELECT jsonb_agg(to_jsonb(p)) FROM profile p WHERE person_id = $1), '[]'::jsonb),
          'teamMemberships', COALESCE((SELECT jsonb_agg(to_jsonb(tm)) FROM team_member tm WHERE person_id = $1), '[]'::jsonb),
          'ownedBusinessProfiles', COALESCE((SELECT jsonb_agg(to_jsonb(bp)) FROM business_profile bp WHERE owner_id = $1), '[]'::jsonb),
          'businessProfileAddresses', COALESCE((SELECT jsonb_agg(to_jsonb(a)) FROM business_profile_address a WHERE business_profile_id IN (SELECT id FROM business_profile WHERE owner_id = $1)), '[]'::jsonb),
          'workouts', COALESCE((SELECT jsonb_agg(to_jsonb(w)) FROM workout w WHERE owner_id = $1), '[]'::jsonb),
          'exercises', COALESCE((SELECT jsonb_agg(to_jsonb(e)) FROM exercise e WHERE owner_id = $1), '[]'::jsonb),
          'workoutExercises', COALESCE((SELECT jsonb_agg(to_jsonb(we)) FROM workout_exercise we WHERE workout_id IN (SELECT id FROM workout WHERE owner_id = $1)), '[]'::jsonb),
          'mediaMetadata', COALESCE((SELECT jsonb_agg(to_jsonb(pm)) FROM person_media pm WHERE person_id = $1), '[]'::jsonb)
        ) AS data
    "#;
    ExportSnapshot::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        [person_id.into()],
    ))
    .one(db)
    .await
    .map_err(|e| e.to_string())?
    .map(|row| row.data)
    .ok_or_else(|| "Could not build workout export snapshot".to_string())
}

pub fn start(db: Arc<DatabaseConnection>) {
    tokio::spawn(async move {
        let mut ticker = interval(TokioDuration::from_secs(30));
        loop {
            ticker.tick().await;
            let jobs = match DataExportGateway::find_pending(&db, 3).await {
                Ok(jobs) => jobs,
                Err(e) => {
                    log::error!("Failed to query data exports: {e}");
                    continue;
                }
            };
            for job in jobs {
                let _ = DataExportGateway::set_processing(&db, job.id).await;
                match build_export(&db, job.person_id, job.uuid.to_string()).await {
                    Ok((key, bytes)) => {
                        if let Err(e) = ImageStorageUseCase::upload_export(&key, bytes).await {
                            let _ = DataExportGateway::set_failed(&db, job.id, e.message).await;
                        } else {
                            let _ = DataExportGateway::set_ready(
                                &db,
                                job.id,
                                key,
                                Utc::now() + Duration::days(7),
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        let _ = DataExportGateway::set_failed(&db, job.id, error).await;
                    }
                }
            }
            if let Ok(expired) = DataExportGateway::find_expired(&db, Utc::now(), 20).await {
                for job in expired {
                    if let Some(key) = job.object_key {
                        if ImageStorageUseCase::delete_presigned_url(key).await.is_ok() {
                            let _ = DataExportGateway::set_expired(&db, job.id).await;
                        }
                    }
                }
            }
        }
    });
}

async fn build_export(
    db: &DatabaseConnection,
    person_id: i32,
    export_uuid: String,
) -> Result<(String, Vec<u8>), String> {
    let person = PersonUseCase::get(db, person_id)
        .await
        .map_err(|e| e.message)?;
    let person_uuid = person
        .uuid
        .clone()
        .ok_or_else(|| "Person UUID missing".to_string())?;
    let consents = ConsentGateway::list(db, person_id)
        .await
        .map_err(|e| e.to_string())?;
    let timeline = TimelineDeletionGateway::export_person_data(&person_uuid)
        .await
        .map_err(|e| e.message)?;
    let related_records = workout_snapshot(db, person_id).await?;
    let workout = json!({
        "person": PersonMapper::json(person),
        "consents": consents.into_iter().map(ConsentJson::from).collect::<Vec<_>>(),
        "relatedRecords": related_records,
    });

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut archive = zip::ZipWriter::new(&mut cursor);
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for (name, data) in [
            ("manifest.json", serde_json::to_vec_pretty(&json!({ "formatVersion": 1, "createdAt": Utc::now(), "personUuid": person_uuid })).unwrap()),
            ("workout.json", serde_json::to_vec_pretty(&workout).map_err(|e| e.to_string())?),
            ("timeline.json", serde_json::to_vec_pretty(&timeline).map_err(|e| e.to_string())?),
        ] {
            archive.start_file(name, options).map_err(|e| e.to_string())?;
            archive.write_all(&data).map_err(|e| e.to_string())?;
        }
        for media in PersonMediaGateway::find_all_by_person(db, person_id)
            .await
            .map_err(|e| e.to_string())?
        {
            let bytes = ImageStorageUseCase::download_object(&media.s3_key)
                .await
                .map_err(|e| e.message)?;
            let safe_name = media
                .s3_key
                .replace("..", "_")
                .trim_start_matches('/')
                .to_string();
            archive
                .start_file(format!("media/{safe_name}"), options)
                .map_err(|e| e.to_string())?;
            archive.write_all(&bytes).map_err(|e| e.to_string())?;
        }
        archive.finish().map_err(|e| e.to_string())?;
    }
    Ok((
        format!("data-exports/{person_uuid}/{export_uuid}.zip"),
        cursor.into_inner(),
    ))
}
