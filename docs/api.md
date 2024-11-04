# Minecraft Server Stack API Documentation

Base endpoint: `/api/v1/stacks`

## Endpoints

### List All Stacks
```http
GET /api/v1/stacks
```

Retrieves a list of all available Minecraft server stacks.

**Curl Example:**
```bash
curl -X GET http://localhost:8080/api/v1/stacks
```

**Response:**
```json
[
    {
        "stack_id": "3",
        "wan_ip": "24.48.49.227",
        "services": {
            "minecraft_server": {
                "port": null,
                "status": "stopped"
            },
            "sftp_server": {
                "port": null,
                "status": "stopped"
            }
        }
    },
    {
        "stack_id": "2",
        "wan_ip": "24.48.49.227",
        "services": {
            "minecraft_server": {
                "port": "4103",
                "status": "running"
            },
            "sftp_server": {
                "port": "4105",
                "status": "running"
            }
        }
    }
]
```

**Status Codes:**
- `200 OK`: List of stacks retrieved successfully
- `204 No Content`: No stacks found
- `500 Internal Server Error`: Retrieval failed

### Create Stack
```http
POST /api/v1/stacks
```

Creates a new Minecraft server stack instance.

**Curl Example:**
```bash
curl -X POST \
  http://localhost:8080/api/v1/stacks \
  -H "Content-Type: application/json"
```

**Response:**
```json
{
    "stack_id": "3",
    "ports": {
        "minecraft_server": "4103",
        "rcon": "4104",
        "sftp_server": "4105"
    }
}
```

**Status Codes:**
- `201 Created`: Stack created successfully
- `403 Forbidden`: Maximum number of stacks reached
- `500 Internal Server Error`: Creation failed

### Delete Stack
```http
DELETE /api/v1/stacks/{stack_id}
```

Removes an existing Minecraft server stack and its associated resources.

**Curl Example:**
```bash
curl -X DELETE http://localhost:8080/api/v1/stacks/3
```

**Parameters:**
- `stack_id` (path parameter): The unique identifier of the stack to delete

**Response:**
- Empty response body

**Status Codes:**
- `204 No Content`: Stack deleted successfully
- `404 Not Found`: Stack not found
- `500 Internal Server Error`: Deletion failed

### Update Stack Status
```http
PATCH /api/v1/stacks/{stack_id}/status
```

Updates the running status of a stack (start/stop).

**Curl Example:**
```bash
curl -X PATCH \
  http://localhost:8080/api/v1/stacks/3/status \
  -H "Content-Type: application/json" \
  -d '{"status": "running"}'
```

**Parameters:**
- `stack_id` (path parameter): The unique identifier of the stack

**Request Body:**
```json
{
    "status": "running" | "stopped"
}
```

**Response:**
- Empty response body

**Status Codes:**
- `204 No Content`: Stack status updated successfully
- `400 Bad Request`: Invalid status value
- `404 Not Found`: Stack not found
- `500 Internal Server Error`: Update failed

## Status Codes Summary

- `200 OK`: Request successful with response body (GET)
- `201 Created`: Resource created successfully (POST)
- `204 No Content`: Request successful with no response body (DELETE, PATCH) or empty list (GET)
- `400 Bad Request`: Invalid request body
- `403 Forbidden`: Maximum number of stacks reached
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server-side error occurred

Each code may include a JSON response body with a message field for error cases, except for 201 (returns resource data) and 204 (no body).

## HTTP Headers

**Request Headers:**
```http
Content-Type: application/json  # For POST and PATCH requests only
```

**Response Headers:**
```http
Content-Type: application/json  # For responses with body (errors, 200, 201)
```

Note: 204 responses (successful DELETE and PATCH operations) do not include any Content-Type header as they have no response body.
