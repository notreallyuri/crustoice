#!/bin/bash

docker inspect voice-project-db --format='{{.State.Running}}' | grep -q true || {
  echo "voice-project-db is not running"
  exit 1
}

docker exec -i voice-project-db psql -U user -d voice_project -f - <db/postgre/002_guilds_down.sql
docker exec -i voice-project-db psql -U user -d voice_project -f - <db/postgre/001_user_down.sql
