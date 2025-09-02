# KPM - Kubernetes Process Manager

A systemd-based process manager for Linux, written in Rust. Think PM2, but using systemd for rock-solid process management.

## Features

- 🚀 **Easy to use**: Simple CLI interface similar to PM2
- 🔧 **systemd integration**: Leverages systemd for robust process management
- 🔄 **Auto-restart**: Processes automatically restart on failure
- 📊 **Process monitoring**: View logs and status of your applications
- 🎯 **Zero configuration**: Works out of the box
- ⚡ **Fast and lightweight**: Built with Rust for performance

## Installation

```bash
cargo install kpm-rs
```

## Quick Start

```bash
# Create a new application
kpm new my-web-server --script "node server.js" --env "PORT=3000"

# Start the application
kpm start my-web-server

# View running applications
kpm list

# View application logs
kpm logs my-web-server

# Follow logs in real-time
kpm logs my-web-server --follow

# Stop application
kpm stop my-web-server

# Restart application
kpm restart my-web-server

# Delete application
kpm delete my-web-server
```

## Usage

### Create Applications

```bash
# Basic application
rpm new myapp --script "node app.js"

# With working directory and environment variables
rpm new webapp --script "npm start" --cwd "/var/www/myapp" --env "NODE_ENV=production" --env "PORT=8080"

# Python application
rpm new api --script "python3 main.py" --env "FLASK_ENV=production"
```

### Manage Applications

```bash
# Start/stop/restart
rpm start myapp
rpm stop myapp
rpm restart myapp

# Stop all applications
rpm stop all

# View detailed info
rpm info myapp

# Monitor all applications
rpm monitor

# Check RPM status
rpm status
```

### View Logs

```bash
# View recent logs
rpm logs myapp

# Follow logs in real-time
rpm logs myapp --follow

# Show specific number of lines
rpm logs myapp --lines 50
```

## How It Works

RPM creates systemd service files for each application you manage. These services are:

- **Automatically started** on system boot
- **Automatically restarted** on failure
- **Integrated with journald** for log management
- **Managed by systemd** for reliability

Each application gets a systemd service named `rpm-{app-name}`.service.

## Examples

### Node.js Web Server

```bash
rpm new web-api --script "node server.js" --cwd "/home/user/myproject" --env "NODE_ENV=production" --env "PORT=3000"
rpm start web-api
```

### Python Flask Application

```bash
rpm new flask-app --script "python3 -m flask run --host=0.0.0.0 --port=5000" --env "FLASK_ENV=production"
rpm start flask-app
```

### Rust Application

```bash
rpm new rust-service --script "./target/release/myapp" --env "RUST_LOG=info"
rpm start rust-service
```

## Configuration

Applications are stored in `~/.rpm/apps.json`. Service files are created in `/etc/systemd/system/`.

## System Requirements

- Linux with systemd
- sudo privileges (for creating systemd services)
- Rust/Cargo (for installation)

## Comparison with PM2

| Feature | PM2 | RPM |
|---------|-----|-----|
| Process Management | ✅ | ✅ |
| Auto-restart | ✅ | ✅ (systemd) |
| Log Management | ✅ | ✅ (journald) |
| Boot Persistence | ✅ | ✅ (systemd) |
| Cluster Mode | ✅ | ❌ |
| Memory Usage | Higher | Lower |
| System Integration | Partial | Full (systemd) |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT OR Apache-2.0 license.