# Minecraft Server Stack API Documentation

Base endpoint: `/api/v1/stacks`

## Endpoints

### List All Stacks
```http
GET /api/v1/stacks
```

Retrieves a list of all available Minecraft server stacks.

**Response:**
```json
{
    "data": {
        "stacks": [
            {
                "services": {
                    "minecraft_server": {
                        "port": null,
                        "status": "stopped"
                    },
                    "sftp_server": {
                        "port": null,
                        "status": "stopped"
                    }
                },
                "stack_id": "3"
            },
            {
                "services": {
                    "minecraft_server": {
                        "port": "4103",
                        "status": "running"
                    },
                    "sftp_server": {
                        "port": "4105",
                        "status": "running"
                    }
                },
                "stack_id": "2"
            }
        ],
        "wan_ip": "24.48.49.227"
    },
    "message": "Stack status retrieved successfully"
}
```

**Status Codes:**
- `200 OK`: Stack status retrieved successfully
- `500 Internal Server Error`: Retrieval failed

### Create Stack
```http
POST /api/v1/stacks
```

Creates a new Minecraft server stack instance.

**Response:**
```json
{
    "data": {
        "ports": {
            "minecraft_server": "4109",
            "rcon": "4110",
            "sftp_server": "4111"
        },
        "stack_id": "4"
    },
    "message": "Stack 4 has been successfully created"
}
```

**Status Codes:**
- `201 Created`: Stack has been successfully created
- `403 Forbidden`: Maximum number of stacks reached
- `500 Internal Server Error`: Creation failed

### Delete Stack
```http
DELETE /api/v1/stacks/{stack_id}
```

Removes an existing Minecraft server stack and its associated resources.

**Parameters:**
- `stack_id` (path parameter): The unique identifier of the stack to delete

**Response:**
- Empty response body

**Status Codes:**
- `204 No Content`: Stack has been successfully deleted
- `404 Not Found`: Stack not found
- `500 Internal Server Error`: Deletion failed

### Update Stack Status
```http
PATCH /api/v1/stacks/{stack_id}/status
```

Updates the running status of a stack (start/stop).

**Parameters:**
- `stack_id` (path parameter): The unique identifier of the stack

**Request Body:**
```json
{
    "status": "running" | "stopped"
}
```

**Response:**
```json
{
    "message": "Stack 1 has been successfully started"
}
```

**Status Codes:**
- `200 OK`: Status updated successfully
- `400 Bad Request`: Invalid status value
- `404 Not Found`: Stack not found
- `500 Internal Server Error`: Status update failed

## Error Responses

All endpoints may return error responses in the following format:

```json
{
    "message": "Error description"
}
```

## Status Codes Summary

- `200 OK`: Request succeeded
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request (malformed data, invalid status)
- `404 Not Found`: Requested resource does not exist
- `500 Internal Server Error`: Server-side error occurred

## HTTP Headers

**Request Headers:**
```http
Content-Type: application/json
```

**Response Headers:**
```http
Content-Type: application/json
```
