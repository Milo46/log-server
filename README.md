# Log Server

**Log Server** is a centralized schema-on-write log sink. Right now, the application
covers very simple functionalities, e.g. creating schemas, log entries and then
retrieving them back to the user. It validates the data structure and makes sure
that data is consistent (every log has it's schema). It supports data transmission
via HTTP and data events via WebSocket also.

## Quickstart

🎯 The current user workflow:

1. 📋 Create/Update log schemas
2. 📤 Push/Update logs continuously to the sink
3. 📥 Retrieve and analyze logs anytime
4. 🔄 Repeat the push/retrieve cycle as needed
5. 🗑️ Delete schemas or individual logs anytime

## Why Log Server?

- ✅ Schema Validation — Ensures data consistency across all logs
- ✅ Centralized — All your logs in one secure place
- ✅ Simple HTTP API — Easy to integrate with any system
- ✅ Data Integrity — Every log is validated against its schema
- ✅ Live data updates - Every log write/deletion is now being pushed via WebSocket

## Prerequisites
- Docker and Docker Compose installed
- Basic understanding of JSON and HTTP requests

## Installation Guide

The project runs on top of `docker compose` and is necessary in order to run the software
in its production and development workflow.

To run the production workflow in the background, run this command:
```bash
docker compose -f docker-compose.yml up -d
```

## Usage Examples

There are two available interfaces:
- pure HTTP requests for writing and reading data
- WebSocket for getting live write updates on the data

### 1. Create your schema.
```bash
curl \
    --request POST \
    --location http://localhost:8080/schemas \
    --header "Content-Type: application/json" \
    --data '{
        "name": "temperature-readings",
        "version": "1.0.0",
        "description": "Logs for the temperature sensors inside my room",
        "schema_definition": {
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "reading": { "type": "number" }
            },
            "required": [ "name", "reading" ]
        }
    }'
```
Response:
```json
{
  "id": "891db49b-4d64-4ba0-b075-156c8c17ce1d",
  "name": "temperature-readings",
  "version": "1.0.0",
  "description": "Logs for the temperature sensors inside my room",
  "schema_definition": {
    "properties": {
      "name": {
        "type": "string"
      },
      "reading": {
        "type": "number"
      }
    },
    "required": [
      "name",
      "reading"
    ],
    "type": "object"
  },
  "created_at": "2025-11-20T20:52:14.548098+00:00",
  "updated_at": "2025-11-20T20:52:14.548098+00:00"
}
```

### 2. Save schema's UUID from the application response. It will be needed to POST logs.

### 3. Create your first log.
```bash
curl \
    --request POST \
    --location http://localhost:8080/logs \
    --header "Content-Type: application/json" \
    --data '{
        "schema_id": "891db49b-4d64-4ba0-b075-156c8c17ce1d",
        "log_data": {
            "name": "desk",
            "reading": 34
        }
    }'
```
Response:
```json
{
  "id": 10,
  "schema_id": "891db49b-4d64-4ba0-b075-156c8c17ce1d",
  "log_data": {
    "name": "desk",
    "reading": 34
  },
  "created_at": "2025-11-20T20:54:59.555233+00:00"
}
```

### 4. Retrieve all your logs.
```bash
curl \
    --request GET \
    --location http://localhost:8080/logs/schema/temperature-readings/1.0.0
```
Response:
```json
{
  "logs": [
    {
      "created_at": "2025-11-20T20:54:59.555233+00:00",
      "id": 10,
      "log_data": {
        "name": "desk",
        "reading": 34
      },
      "schema_id": "891db49b-4d64-4ba0-b075-156c8c17ce1d"
    }
  ]
}
```

## Listening to events via WebSocket

In order to get live updates on the logs, you have to somehow get
notified by the server and you can achieve it by connecting to the
WebSocket endpoint of the application.

```bash
# If you want to listen to all logs
websocat "ws://localhost:8081/ws/logs"

# And if you want to listen to only a specific schema
websocat "ws://localhost:8081/ws/logs?schema_id=0a9dadf1-fd1b-4727-88d5-98aad5ce70a3"
```

**Note**: If you provide an invalid or non-existent `schema_id`,
the WebSocket connection will fail with a `404 Not Found` error:
```bash
websocat: WebSocketError: Received unexpected status code (404 Not Found)
```

Make sure the schema exists before attempting to connect.

The following events are currently supported:

### 1. Log creation message
```json
{
    "event_type": "created",
    "id": 5826,
    "schema_id": "0a9dadf1-fd1b-4727-88d5-98aad5ce70a3",
    "log_data": {
        "message":"Hello World from the working WebSocket connection!"
    },
    "created_at": "2025-12-05T11:13:36.361797+00:00"
}
```

### 2. Log deletion message
```json
{
    "event_type": "deleted",
    "id": 5826,
    "schema_id": "0a9dadf1-fd1b-4727-88d5-98aad5ce70a3"
}
```

## Features
## Configuration
## License
