# Claw Extensions

**Advanced features for the claw-core cellular agent engine**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/claw-extensions)

---

## Overview

`claw-extensions` provides advanced, optional features for the `claw-core` cellular agent engine. While `claw-core` focuses on being a minimal MVP (<5,000 LOC), this repository contains all the sophisticated features for production-scale agent systems.

### Relationship to claw-core

```
┌─────────────────────────────────────────────────────────────────┐
│                          claw-core                               │
│                    (Minimal MVP <5,000 LOC)                      │
│  • Basic Agent lifecycle                                         │
│  • Single Memory equipment slot                                  │
│  • Simple REST API (5 endpoints)                                 │
│  • Basic trigger processing                                      │
└───────────────────────┬─────────────────────────────────────────┘
                        │ Extends via
                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                       claw-extensions                            │
│              (Advanced Features for Production)                  │
│  • Multi-slot equipment system (6 slots)                         │
│  • Social coordination (master-slave, co-worker)                 │
│  • Seed learning & model distillation                            │
│  • Bot automation loops                                          │
│  • WebSocket real-time communication                             │
│  • GPU acceleration (CUDA/WGPU)                                  │
│  • Advanced monitoring & telemetry                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Available Extensions

### 1. Advanced Equipment System

**Location:** `extensions/equipment/`

Extends the single Memory slot in claw-core to 6 equipment slots:

- **MEMORY** - Hierarchical memory with L0/L1/L2 caching
- **REASONING** - Escalation engine for complex decisions
- **CONSENSUS** - Tripartite consensus for multi-agent agreement
- **SPREADSHEET** - Tile interface for spreadsheet integration
- **DISTILLATION** - Model compression and optimization
- **COORDINATION** - Swarm coordination for parallel processing

**Features:**
- Equipment hot-swapping
- Muscle memory extraction
- Cost/benefit analysis
- Lazy loading
- Equipment monitoring

**Example:**
```rust
use claw_extensions::equipment::{EquipmentManager, EquipmentSlot};
use claw_extensions::equipment::reasoning::EscalationEngine;

// Create equipment manager
let mut manager = EquipmentManager::new();

// Equip multiple slots
manager.equip(EquipmentSlot::Memory, Box::new(HierarchicalMemory::new())).await?;
manager.equip(EquipmentSlot::Reasoning, Box::new(EscalationEngine::new())).await?;
manager.equip(EquipmentSlot::Consensus, Box::new(TripartiteConsensus::new())).await?;

// Process with all equipped items
let result = manager.process(data).await?;
```

---

### 2. Social Coordination

**Location:** `extensions/social/`

Multi-agent coordination patterns for complex workflows:

**Patterns:**
- **Master-Slave** - Parallel processing coordination
- **Co-Worker** - Peer collaboration
- **Peer** - Equal coordination
- **Delegate** - Task delegation
- **Observer** - Monitoring relationship

**Coordination Strategies:**
- `PARALLEL` - Execute simultaneously, aggregate
- `SEQUENTIAL` - Execute in order
- `CONSENSUS` - All must agree
- `MAJORITY_VOTE` - Majority wins
- `WEIGHTED` - Weight by confidence

**Example:**
```rust
use claw_extensions::social::{SocialManager, CoordinationPattern};
use claw_extensions::social::patterns::MasterSlave;

// Create social manager
let mut social = SocialManager::new();

// Create master-slave relationship
let pattern = MasterSlave::new(
    master_agent_id,
    vec![slave1_id, slave2_id, slave3_id],
    CoordinationStrategy::Parallel,
);

social.register_pattern(pattern).await?;

// Coordinate parallel work
let result = social.coordinate(master_agent_id, task_data).await?;
```

---

### 3. Seed Learning System

**Location:** `extensions/learning/`

Machine learning behavior optimization for agents:

**Features:**
- Seed definition (natural language behavior)
- Training algorithms
- Model distillation
- Behavior optimization
- Learning metrics tracking

**Example:**
```rust
use claw_extensions::learning::{Seed, LearningStrategy, SeedTrainer};

// Define a seed
let seed = Seed::new(
    "Monitor temperature and detect anomalies",
    LearningStrategy::Reinforcement,
);

// Train the seed
let trainer = SeedTrainer::new();
let learned_claw = trainer.train(
    seed,
    training_data,
    TrainingConfig::default()
).await?;

// Deploy learned behavior
agent.deploy_learned_behavior(learned_claw).await?;
```

---

### 4. Bot Automation

**Location:** `extensions/bot/`

Simple automation loops WITHOUT ML models:

**Features:**
- Automated loops
- Polling mechanisms
- Simple triggers
- Scheduled tasks
- Event-driven automation

**Example:**
```rust
use claw_extensions::bot::{Bot, BotConfig, Loop};

// Create a polling bot
let config = BotConfig::new(
    "sensor-poller",
    Loop::Interval(Duration::from_secs(5)),
);

let mut bot = Bot::new(config);

// Define automation loop
bot.register_loop(|_| async {
    let data = fetch_sensor_data().await?;
    if data.temperature > threshold {
        trigger_alert().await?;
    }
    Ok(())
}).await?;

bot.start().await?;
```

---

### 5. WebSocket Server

**Location:** `extensions/websocket/`

Real-time bidirectional communication:

**Features:**
- WebSocket connection management
- Message routing
- Authentication
- Broadcast channels
- Connection pooling

**Example:**
```rust
use claw_extensions::websocket::{WebSocketServer, WsConfig};

// Create WebSocket server
let config = WsConfig::new("127.0.0.1:8080");
let mut ws_server = WebSocketServer::new(config).await?;

// Handle connections
ws_server.on_connection(|mut socket| async move {
    while let Some(msg) = socket.recv().await? {
        // Handle message
        socket.send(response).await?;
    }
    Ok(())
}).await?;

ws_server.serve().await?;
```

---

### 6. GPU Acceleration

**Location:** `extensions/gpu/`

High-performance parallel processing:

**Backends:**
- CUDA (NVIDIA GPUs)
- WGPU (cross-platform)

**Features:**
- Parallel agent execution
- Batch processing
- GPU memory management
- Kernel optimization

**Example:**
```rust
use claw_extensions::gpu::{GpuExecutor, GpuConfig};

// Create GPU executor
let config = GpuConfig::cuda(device_id)?;
let mut executor = GpuExecutor::new(config)?;

// Submit batch work
let results = executor.process_batch(agents_data).await?;
```

---

### 7. Advanced Monitoring

**Location:** `extensions/monitoring/`

Production-grade observability:

**Features:**
- Metrics collection
- Performance profiling
- Health checking
- Telemetry
- Distributed tracing

**Example:**
```rust
use claw_extensions::monitoring::{Monitor, MetricsCollector};

// Create monitor
let monitor = Monitor::new();

// Track metrics
monitor.track_metric("agent_processing_time", duration_ms);
monitor.track_metric("memory_usage", bytes);
monitor.track_metric("active_agents", count);

// Get health status
let health = monitor.health_check().await?;
```

---

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
claw-core = "0.1"
claw-extensions = "0.1"

# Enable specific extensions
claw-extensions = { version = "0.1", features = ["equipment", "social"] }
```

### Feature Flags

- `equipment` - Advanced equipment system
- `social` - Social coordination
- `learning` - Seed learning system
- `bot` - Bot automation
- `websocket` - WebSocket server
- `gpu` - GPU acceleration
- `monitoring` - Advanced monitoring
- `full` - Enable all extensions

---

## Quick Start

### Basic Extension Usage

```rust
use claw_core::{Agent, MinimalAgent, AgentConfig};
use claw_extensions::equipment::EquipmentManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create base agent from claw-core
    let config = AgentConfig {
        id: "my-agent".to_string(),
        cell_ref: "A1".to_string(),
        model: "deepseek-chat".to_string(),
        equipment: vec![],
        config: Default::default(),
    };

    let mut agent = MinimalAgent::new(config);

    // Extend with equipment manager
    let mut equipment_mgr = EquipmentManager::new();
    equipment_mgr.equip(
        claw_extensions::equipment::EquipmentSlot::Memory,
        Box::new(claw_extensions::equipment::memory::HierarchicalMemory::new())
    ).await?;

    // Agent now has advanced equipment
    agent.set_equipment_manager(equipment_mgr);

    Ok(())
}
```

### With Social Coordination

```rust
use claw_extensions::social::{SocialManager, patterns::MasterSlave};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut social = SocialManager::new();

    // Create master-slave pattern
    let pattern = MasterSlave::new(
        "master-agent",
        vec!["slave-1", "slave-2", "slave-3"],
        claw_extensions::social::CoordinationStrategy::Parallel,
    );

    social.register_pattern(pattern).await?;

    // Coordinate work
    let result = social.coordinate("master-agent", task_data).await?;

    Ok(())
}
```

---

## Architecture

### Extension Loading

Extensions are designed to be optional and composable:

```rust
// Core agent (required)
let mut agent = MinimalAgent::new(config);

// Add extensions incrementally
if cfg!(feature = "equipment") {
    agent.add_extension(EquipmentManager::new());
}

if cfg!(feature = "social") {
    agent.add_extension(SocialManager::new());
}

if cfg!(feature = "monitoring") {
    agent.add_extension(Monitor::new());
}
```

### Extension Interface

All extensions implement the `Extension` trait:

```rust
#[async_trait]
pub trait Extension: Send + Sync {
    fn name(&self) -> &str;

    async fn initialize(&mut self, agent: &mut dyn Agent) -> Result<()>;
    async fn process(&mut self, message: &Message) -> Result<Option<Message>>;
    async fn shutdown(&mut self) -> Result<()>;
}
```

---

## Documentation

- [Equipment System Guide](./docs/equipment.md)
- [Social Coordination Guide](./docs/social.md)
- [Learning System Guide](./docs/learning.md)
- [Bot Automation Guide](./docs/bot.md)
- [WebSocket Guide](./docs/websocket.md)
- [GPU Acceleration Guide](./docs/gpu.md)
- [Monitoring Guide](./docs/monitoring.md)

---

## Examples

See the `examples/` directory for complete examples:

- `examples/basic_equipment.rs` - Using advanced equipment
- `examples/social_coordination.rs` - Multi-agent coordination
- `examples/seed_learning.rs` - Training and deploying seeds
- `examples/bot_automation.rs` - Creating automated bots
- `examples/websocket_server.rs` - Real-time communication
- `examples/gpu_acceleration.rs` - GPU-accelerated processing
- `examples/full_stack.rs` - Using all extensions together

---

## Performance

### Benchmarks

| Feature | Latency | Throughput | Memory |
|---------|---------|------------|--------|
| Equipment hot-swap | ~1ms | 1000/sec | ~2MB |
| Social coordination | ~5ms | 200/sec | ~5MB |
| Seed training | N/A | N/A | ~50MB |
| WebSocket message | ~0.5ms | 2000/sec | ~1MB |
| GPU batch | ~10ms | 100/sec | ~100MB |

See [BENCHMARKS.md](./BENCHMARKS.md) for details.

---

## Testing

```bash
# Run all tests
cargo test --all-features

# Run specific extension tests
cargo test --package claw-extensions --features equipment
cargo test --package claw-extensions --features social

# Run with coverage
cargo tarpaulin --all-features --out Html
```

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone repository
git clone https://github.com/SuperInstance/claw-extensions.git
cd claw-extensions

# Install dependencies
cargo fetch

# Run development build
cargo build --all-features

# Run tests
cargo test --all-features
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

Built on top of [claw-core](https://github.com/SuperInstance/claw), the minimal cellular agent engine.

Part of the [SuperInstance](https://superinstance.ai) ecosystem for cellular agent infrastructure.

---

## Status

**Version:** 0.1.0 (Alpha)

**Stability:** Experimental - API may change

**Production Ready:** No - Use at your own risk

---

**Last Updated:** 2026-03-18
