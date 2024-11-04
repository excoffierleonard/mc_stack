# Minecraft Server Stack Manager

A robust, containerized Minecraft server management solution built with Rust and Docker. This service provides an intuitive web interface for creating and managing multiple Minecraft server instances, each with its own SFTP access and RCON capabilities.

## 📚 Table of Content

- [Features](#-features)
- [Technical Stack](#️-technical-stack)
- [Prerequisites](#-prerequisites)
- [Quick Start](#-quick-start)
- [Development](#-development)
- [Configuration](#-configuration)
- [API Documentation](#-api-documentation)
- [Roadmap](#️-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Features

- **Multi-Server Management**: Create and manage multiple Minecraft server instances dynamically
- **Container Isolation**: Each server runs in its own isolated Docker container
- **Resource Control**: Automatic CPU-based scaling limits
- **Web Interface**: Modern, responsive UI for server management
- **Integrated Services**:
  - 🎮 Minecraft Server
  - 📁 SFTP Server for file access
  - 🎛️ RCON support for remote commands
- **Status Management**: Start, stop, and monitor server status
- **Port Management**: Automatic port allocation and management

## 🛠️ Technical Stack

- **Backend**: Rust with Actix-web
- **Frontend**: HTML/JavaScript with Tailwind CSS
- **Containerization**: Docker with docker-compose
- **Storage**: Docker volumes for persistence
- **API**: RESTful JSON API

## 📦 Prerequisites

- Docker and Docker Compose
- Rust 1 (for development)

## ⚡ Quick Start

Download the [compose.yaml](compose.yaml) file and start the service:

```bash
curl -o compose.yaml https://git.jisoonet.com/el/mc_stack/raw/branch/main/compose.yaml && docker compose up -d
```

The web interface will be available at `http://localhost:8080`

## 💻 Development

Run the service locally:
```bash
cargo run
```

Build for production:
```bash
cargo build --release
```

## 🔧 Configuration

- Service runs on `0.0.0.0:8080`
- Requires Docker socket mounted at `/var/run/docker.sock`
- Stack limits based on available CPU cores
- Automatic port increment: 3 ports per stack (Minecraft, RCON, SFTP)

## 📖 API Documentation

Comprehensive API documentation is available in [docs/api.md](docs/api.md), including:
- Stack creation and management
- Status updates
- Server listing
- Error handling

## 🗺️ Roadmap

- [ ] Backup system implementation using duplicacy
- [ ] WebAssembly migration for web interface
- [ ] Direct Docker API integration
- [ ] Enhanced container status monitoring
- [ ] Docker command introspection improvements

## 🤝 Contributing

We welcome contributions! Please feel free to submit Pull Requests.

## 📝 License

This project is licensed under the GNU AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.

**Commercial Use**: For commercial licensing options, please contact [Your Contact Info].

---
For detailed API usage and endpoints, see our [API Documentation](docs/api.md).