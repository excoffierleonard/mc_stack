# Minecraft Server Stack

## Getting Started
```bash
git clone https://git.jisoonet.com/el/mc_stack.git && \
cd mc_stack && \
sudo chmod +x scripts/*.sh
```

## Usage

### Create a new stack
```bash
./scripts/create_stack.sh
```

### Delete the stack
```bash
./scripts/delete_stack.sh <stack_id>
```

### Start the stack
```bash
./scripts/start_stack.sh <stack_id>
```

### Stop the stack
```bash
./scripts/stop_stack.sh <stack_id>
```

### List the stacks
```bash
./scripts/list_stacks.sh
```

## Need to implement a backup mechanism using duplicacy