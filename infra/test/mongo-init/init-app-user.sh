#!/bin/bash
set -e
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
