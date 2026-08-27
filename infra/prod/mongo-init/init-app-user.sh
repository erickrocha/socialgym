#!/bin/bash
set -e
# Runs once, on a fresh data volume, against the temporary no-auth mongod.
# Creates the application user in MONGODB_DATABASE (= TIMELINE_DATABASE_NAME).
# The app then connects as this user with authSource=MONGODB_DATABASE.
mongosh <<EOF
use $MONGODB_DATABASE
if (!db.getUser("$MONGODB_USERNAME")) {
  db.createUser({
    user: "$MONGODB_USERNAME",
    pwd: "$MONGODB_PASSWORD",
    roles: [{ role: "readWrite", db: "$MONGODB_DATABASE" }]
  })
}
EOF
