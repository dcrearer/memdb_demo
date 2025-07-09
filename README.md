# MemoryDB Demo

A Rust demonstration project showcasing AWS MemoryDB for Redis integration using the AWS SDK.

## Overview

This project demonstrates how to interact with AWS MemoryDB for Redis using Rust. It provides a simple example of creating and managing MemoryDB clusters programmatically.

## Features

- AWS MemoryDB cluster creation and management
- Configuration-driven setup using YAML
- Structured logging with tracing
- Async/await support with Tokio runtime

## Prerequisites

- Rust (2024 edition)
- AWS credentials configured (via AWS CLI, environment variables, or IAM roles)
- Appropriate AWS permissions for MemoryDB operations

## Dependencies

- `aws-sdk-memorydb` - AWS SDK for MemoryDB operations
- `aws-config` - AWS configuration management
- `tokio` - Async runtime
- `config` - Configuration file parsing
- `serde` - Serialization/deserialization
- `anyhow` - Error handling
- `tracing` - Structured logging

## Project Structure

```
memdb_demo/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library root
│   ├── configuration.rs  # Configuration management
│   └── startup.rs        # Application startup logic
├── configuration/
│   └── config.yaml       # Configuration file
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Configuration

The application uses a YAML configuration file located at `configuration/config.yaml`. Modify this file to customize your MemoryDB settings.

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/dcrearer/memdb_demo.git
   cd memdb_demo
   ```

2. Ensure your AWS credentials are configured:
   ```bash
   aws configure
   ```

3. Build and run the project:
   ```bash
   cargo run
   ```

## Output

The application will:
- Initialize logging
- Load configuration from the YAML file
- Create a MemoryDB cluster
- Display cluster information including name, engine, and ARN

## License

This project is a demonstration and is provided as-is for educational purposes.

## Contributing

This is a demo project. Feel free to fork and modify for your own learning purposes.
