#!/usr/bin/bash

IMAGE_NAME="rust-backend-demo-db-image"
CONTAINER_NAME="rust-backend-demo-db"

if docker ps --filter "name=$CONTAINER_NAME" --filter "status=running" | grep -q "$CONTAINER_NAME"; then
    echo "Container '$CONTAINER_NAME' is running. removing it"
    docker rm -f "$CONTAINER_NAME"
fi

echo "Container '$CONTAINER_NAME' is not running. Starting it now..."
docker build -t "$IMAGE_NAME" .
docker run -d --name "$CONTAINER_NAME" -p 5432:5432 -e POSTGRES_PASSWORD=password "$IMAGE_NAME" || echo "Failed to start container '$CONTAINER_NAME'."
