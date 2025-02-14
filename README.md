# EdgeLogger: Industrial Log Ingestion Platform

A learning project to build a high-performance log ingestion server for industrial machine data, exploring modern technologies and best practices in systems programming.

## Project Overview

EdgeLogger is a log ingestion server designed to receive, process, and store telemetry data from industrial machines. The project serves as a practical learning ground for:

- Systems programming with Rust
- Message queuing with Apache Kafka
- Container orchestration with Kubernetes
- Observability with Prometheus/Grafana

## Core Features

- TCP server for receiving machine telemetry data
- Asynchronous message processing
- Persistent storage in PostgreSQL
- Message queue integration with Kafka
- Basic data validation and timestamp handling
- HTTP API for querying logs
- Monitoring and observability

## Tech Stack

- **Backend**: Rust
- **Database**: PostgreSQL
- **Message Queue**: Apache Kafka
- **Monitoring**: Prometheus/Grafana
- **Deployment**: Kubernetes

## Project Goals

1. **Performance**: Build a high-performance log ingestion system
2. **Reliability**: Ensure robust handling of machine data
3. **Observability**: Implement comprehensive monitoring
4. **Learning**: Gain practical experience with:
   - Rust async programming
   - Database integration
   - Message queue systems
   - Container orchestration
   - Monitoring and metrics

## Example Data Format

```json
{
  "machine_id": "machine-001",
  "timestamp": "2025-02-09T10:00:00Z",
  "metrics": {
    "temperature": 35.5,
    "pressure": 100.2
  }
}
```

## Current Status

Project is in early development, focusing on:

- TCP server implementation
- Database integration
- Basic message handling

## Future Plans

- Edge client implementation
- Advanced analytics
- User management
- Security enhancements
- High availability

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.