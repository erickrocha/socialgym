-- Fixtures for the Timeline acceptance tests that call Workout over gRPC (C-006 TC-007..TC-009).
-- Idempotent; applied by start.sh after Workout has migrated. Acceptance tests that call
-- Migrator::refresh on this database wipe it, so re-run `./start.sh seed` afterwards.
--
--   person 1 / user ...061  "Sender Person"   notificationsEnabled = true,  all consents
--   person 2 / user ...062  "Receiver Person" notificationsEnabled = false, all consents
--   person 99 is deliberately absent -> SettingsService returns NOT_FOUND
--   persons 1 and 2 are accepted friends (mention eligibility)
--
-- Rows above use fixed ids, which do not advance the id sequences, so the block at the end moves every
-- sequence past MAX(id); otherwise the next signup/insert without an id collides with a seeded row.

INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
  (1, '00000000-0000-0000-0000-000000000061', 'Sender',   'Person', '1990-01-01', 'X', now(), now()),
  (2, '00000000-0000-0000-0000-000000000062', 'Receiver', 'Person', '1990-01-01', 'X', now(), now())
ON CONFLICT DO NOTHING;

INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at) VALUES
  (1, '10000000-0000-0000-0000-000000000061', 'Sender Person',   'c006-sender@example.test',   'unused', false, true, 1, '00000000-0000-0000-0000-000000000061', now(), now()),
  (2, '10000000-0000-0000-0000-000000000062', 'Receiver Person', 'c006-receiver@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000062', now(), now())
ON CONFLICT DO NOTHING;

INSERT INTO settings (uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at) VALUES
  ('40000000-0000-0000-0000-000000000061', 1, '00000000-0000-0000-0000-000000000061', 'en', 'light', true,  'Left', 'feed', now(), now()),
  ('40000000-0000-0000-0000-000000000062', 2, '00000000-0000-0000-0000-000000000062', 'en', 'light', false, 'Left', 'feed', now(), now())
ON CONFLICT (uuid) DO UPDATE SET notifications_enabled = EXCLUDED.notifications_enabled, created_at = EXCLUDED.created_at;

INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
SELECT md5('consent-' || p.id || '-' || d.document)::uuid, p.id, d.document, '1.0.0', now(), '127.0.0.1'
FROM (VALUES (1), (2)) AS p(id),
     (VALUES ('terms'), ('privacy'), ('health_data')) AS d(document)
ON CONFLICT DO NOTHING;

INSERT INTO friends (uuid, person_id, person_uuid, friend_id, friend_uuid, status, created_at, updated_at) VALUES
  ('60000000-0000-0000-0000-000000000001', 1, '00000000-0000-0000-0000-000000000061', 2, '00000000-0000-0000-0000-000000000062', 'Accepted', now(), now()),
  ('60000000-0000-0000-0000-000000000002', 2, '00000000-0000-0000-0000-000000000062', 1, '00000000-0000-0000-0000-000000000061', 'Accepted', now(), now())
ON CONFLICT DO NOTHING;

-- Advance every id sequence (serial and identity columns) past the highest seeded id.
DO $$
DECLARE r record;
BEGIN
  FOR r IN
    SELECT c.table_name, c.column_name,
           pg_get_serial_sequence(quote_ident(c.table_name), c.column_name) AS seq
    FROM information_schema.columns c
    WHERE c.table_schema = 'public'
  LOOP
    IF r.seq IS NOT NULL THEN
      EXECUTE format('SELECT setval(%L, COALESCE((SELECT MAX(%I) FROM %I), 0) + 1, false)',
                     r.seq, r.column_name, r.table_name);
    END IF;
  END LOOP;
END $$;
