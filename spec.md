# EdgeLogger: Industrial Log Ingestion Platform

## Project Goal
Build a log ingestion server that can receive, store, and provide access to industrial machine data, while learning:
- Rust programming
- Apache Kafka for message queuing
- Kubernetes for deployment
- Prometheus/Grafana for observability

## Core Functionality
1. **Data Reception**
   - TCP server to receive machine data
   - Simple JSON format for messages
   - Basic authentication

2. **Data Processing**
   - Store messages in Kafka
   - Basic data validation
   - Timestamp handling

3. **Data Storage**
   - PostgreSQL for persistent storage
   - Basic data retention policies

4. **Data Access**
   - Simple HTTP API to query logs
   - Basic filtering by time/machine

5. **Monitoring**
   - Prometheus metrics
   - Grafana dashboard
   - Basic health checks

## Technology Stack
- Rust (backend server)
- Apache Kafka (message queue)
- PostgreSQL (storage)
- Prometheus/Grafana (monitoring)
- Kubernetes (deployment)

## Development Phases

### Phase 1: Basic Server (Learning Rust)
- TCP server implementation
- JSON message handling
- Basic data storage
- Simple HTTP API

### Phase 2: Message Queue (Learning Kafka)
- Kafka integration
- Message publishing
- Consumer implementation

### Phase 3: Monitoring (Learning Observability)
- Prometheus metrics
- Grafana dashboard
- Health checks

### Phase 4: Deployment (Learning Kubernetes)
- Containerization
- Kubernetes deployment
- Basic scaling

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

## Initial Project Structure
```
edgelogger/
├── src/
│   ├── main.rs
│   ├── server.rs     # TCP server
│   ├── handler.rs    # Message handling
│   └── storage.rs    # Data storage
├── Cargo.toml
└── README.md
```

## Success Criteria
- Can receive and store machine data
- Basic querying capability
- Observable metrics
- Runs in Kubernetes

## Not In Initial Scope
- Edge client implementation
- Complex analytics
- User management
- Advanced security
- High availability
- Data transformation