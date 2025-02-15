#!/bin/bash

DB_NAME="example"
DB_USER="admin_example"
DB_PASSWORD="example_pwd"

echo "--- postgres DB creation... ---"

if ! command -v psql &> /dev/null
then
  echo "Posgresql in not installed"
  exit 1
fi 

echo "User creation..."
psql postgres -tc "SELECT 1 FROM pg_roles WHERE rolname = '$DB_USER';" | grep -q 1 || \
psql postgres -c "CREATE ROLE $DB_USER WITH LOGIN PASSWORD '$DB_PASSWORD';"


echo "db creation..."
psql postgres -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME';" | grep -q 1 || \
psql postgres -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;"

echo "bd created and up"

psql postgres -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

echo "--- end of DB config ---"


