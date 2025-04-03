#!/usr/bin/bash

CONTAINER_NAME="postgres"

if docker ps --filter "name=$CONTAINER_NAME" --filter "status=running" | grep -q "$CONTAINER_NAME"; then
    echo "Container '$CONTAINER_NAME' is running. removing it"
    docker rm -f "$CONTAINER_NAME"
fi

echo "Container '$CONTAINER_NAME' is not running. Starting it now..."
# alternative: docker start
docker run --name "$CONTAINER_NAME" -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres || echo "Failed to start container '$CONTAINER_NAME'."

export PASSWORD="password"
while true; do
    if nc -z localhost 5432 >/dev/null 2>&1; then
        echo "db is accepting connections on localhost:5432. verifying readiness..."
        # Check if PostgreSQL is fully ready by running a simple query
        if PGPASSWORD="$PASSWORD" psql -h localhost -U postgres -d postgres -c "SELECT 1" >/dev/null 2>&1; then
            echo "PostgreSQL is ready! Continuing script..."
            break
        fi
    fi
    sleep 1
done

psql -h localhost -U postgres -d postgres -f init.sql
