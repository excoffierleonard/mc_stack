# Minecraft Server Stack

## Getting Started

```bash
git clone https://git.jisoonet.com/el/mc_stack.git && \
cd mc_stack
```

## API

The service provides REST endpoints for managing Minecraft server stacks.

### Endpoints

#### Create a New Stack

```
POST /api/v1/create
```

Creates a new Minecraft server stack instance.

**Response:**

- `200 OK`: Stack created successfully
- Output contains the creation process details and stack identifier

#### Delete a Stack

```
DELETE /api/v1/{stack_id}
```

Removes an existing Minecraft server stack and its associated resources.

**Parameters:**

- `stack_id` (path parameter): The unique identifier of the stack to delete

**Response:**

- `200 OK`: Stack deleted successfully
- Output contains the deletion process details

#### Start a Stack

```
PUT /api/v1/{stack_id}
```

Starts a stopped Minecraft server stack.

**Parameters:**

- `stack_id` (path parameter): The unique identifier of the stack to start

**Response:**

- `200 OK`: Stack started successfully
- Output contains the startup process details

#### Stop a Stack

```
POST /api/v1/{stack_id}
```

Stops a running Minecraft server stack.

**Parameters:**

- `stack_id` (path parameter): The unique identifier of the stack to stop

**Response:**

- `200 OK`: Stack stopped successfully
- Output contains the shutdown process details

#### List All Stacks

```
GET /api/v1/list
```

Retrieves a list of all available Minecraft server stacks.

**Response:**

- `200 OK`: List retrieved successfully
- Output contains the list of stacks and their details

#### Static Files

```
GET /
```

Serves static files from the `static/` directory. The default page is `index.html`.

### Example API Usage

```bash
# Create a new stack
curl -X POST http://localhost:8080/api/v1/create

# Start a specific stack
curl -X PUT http://localhost:8080/api/v1/123

# Stop a specific stack
curl -X POST http://localhost:8080/api/v1/123

# Delete a specific stack
curl -X DELETE http://localhost:8080/api/v1/123

# List all stacks
curl http://localhost:8080/api/v1/list
```

## Dev

```bash
cargo run
```

## Prod

### Build and Run

```bash
cargo build --release && \
pkill mc_stack && \
nohup target/release/mc_stack &> output.log &
```

### Stop

```bash
pkill mc_stack
```

### Docker

```bash
docker build -t git.jisoonet.com/el/mc_stack . && \
docker push git.jisoonet.com/el/mc_stack
```

```bash
docker compose up -d
```

## Todo

- Implement a backup mechanism using duplicacy
- Migrate everything to rust, the static web files may be converted to webassembly

## Notes

- The service runs on `0.0.0.0:8080`
