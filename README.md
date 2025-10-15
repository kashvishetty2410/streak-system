# 🏆 Streakinator3000

> Build unbreakable habits with blockchain-powered daily streaks on Stellar

## 🌟 Vision

Streakinator3000 revolutionizes habit formation by combining gamification with blockchain technology. Built on the Stellar blockchain using Soroban smart contracts, it provides an immutable, transparent way to track and reward daily streaks, making habit-building both fun and accountable.

## ✨ Features

- 🔗 Blockchain-powered streak tracking
- 🎮 Gamification elements
- 🏅 Achievement system
- 💰 Token rewards
- 🔐 Secure smart contracts
- 🌐 Web interface

```
Daily Streak Progress:
[⭐][⭐][⭐][⭐][⭐] Level 1 Complete!
[⭐][⭐][⭐][🔒][🔒] Level 2 In Progress...
```

## 🚀 Installation

### Prerequisites

- Rust (latest stable)
- Node.js (for Soroban CLI)
- Cargo and rustup
- WASM target (for frontend)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install trunk (for frontend)
cargo install trunk

# Install Soroban CLI
cargo install soroban-cli
```

### Smart Contract Deployment

```bash
cd smart-contract
soroban contract deploy --network testnet --source <your-secret-key>
```

### Backend Setup

```bash
# Clone repository
git clone https://github.com/yourusername/streakinator3000
cd streakinator3000

# Build and run backend
cargo build -p streak_backend
cargo run -p streak_backend
```

### Frontend Setup (Optional)

```bash
cd frontend
trunk serve --open
```

## 💫 Usage

1. Connect your Stellar wallet
2. Start a new streak
3. Check in daily to maintain your streak
4. Earn rewards as you progress

```rust
// Example: Check in for the day
pub async fn check_in(&self) -> Result<bool, Error> {
    let today = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() / 86400;
    
    self.contract.check_in(today).await
}
```

## 📁 Project Structure

```
streakinator3000/
├── smart-contract/     # Soroban smart contract
├── backend/           # Rust backend service
├── frontend/         # Yew-based WASM frontend
├── docs/            # Documentation
└── tests/           # Integration tests
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


