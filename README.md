# Prediction Markets Platform

A decentralized prediction markets platform built on Solana blockchain using the Anchor framework. This platform enables users to create, trade, and resolve prediction markets with automated market making using the Logarithmic Market Scoring Rule (LMSR).

## 🚀 Features

- **Decentralized Prediction Markets**: Create and participate in prediction markets on any binary outcome
- **Automated Market Making**: LMSR-based pricing for continuous liquidity
- **Tokenized Shares**: Each market outcome is represented by tradeable SPL tokens
- **Admin Controls**: Configurable platform fees and treasury management
- **User Profiles**: Track betting history, profits, and account balances
- **Market Resolution**: Automated outcome determination with deadline enforcement

### Core Components

- **Market**: Represents a prediction market with YES/NO outcomes
- **Bettor**: User profile with betting history and balance tracking
- **Wager**: Individual bet records linking users to markets
- **Platform Config**: Admin settings, fees, and treasury management

### Smart Contract Structure

```
prediction-markets/
├── src/
│   ├── instructions/          # Program instructions
│   │   ├── create_market.rs   # Market creation
│   │   ├── buy_shares.rs      # Purchase outcome shares
│   │   ├── sell_shares.rs     # Sell outcome shares
│   │   ├── resolve.rs         # Market resolution
│   │   ├── init_config.rs     # Platform initialization
│   │   └── ...
│   ├── state/                 # Data structures
│   │   ├── market.rs          # Market state
│   │   ├── bettor.rs          # User profiles
│   │   ├── wager.rs           # Bet records
│   │   └── platform_config.rs # Platform settings
│   ├── helper/                # Utility functions
│   │   ├── helper.rs          # Core LMSR logic
│   │   └── macros.rs          # Custom macros
│   └── lib.rs                 # Program entry point
```

## 🛠️ Technology Stack

- **Blockchain**: Solana
- **Framework**: Anchor 0.31.1
- **Language**: Rust
- **Token Standard**: SPL Token 2022
- **Metadata**: Metaplex Token Metadata
- **Pricing**: Logarithmic Market Scoring Rule (LMSR)

## 📋 Prerequisites

- Rust 1.70+
- Solana CLI 1.17+
- Anchor CLI 0.31+
- Node.js 18+
- Yarn

## 🚀 Installation & Setup

### 1. Clone the Repository

```bash
git clone <repository-url>
cd prediction-markets
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies
yarn install
```

### 3. Build the Program

```bash
anchor build
```

### 4. Run Tests

```bash
anchor test
```

## 🔧 Configuration

### Environment Setup

1. **Solana Configuration**: Ensure your Solana CLI is configured for the target network
2. **Wallet Setup**: Configure your wallet path in `Anchor.toml`
3. **Network Selection**: Choose between `localnet`, `devnet`, or `mainnet`

### Platform Configuration

The platform requires initialization with:
- Admin public keys
- Platform fees (basis points)
- Treasury configuration

## 📖 Usage

### Creating a Market

```typescript
// Market creation parameters
const marketArgs = {
  name: "Will BTC reach $100k by end of 2024?",
  description: "Binary outcome market for Bitcoin price prediction",
  lsmr_b: 10000, // Liquidity parameter
  dead_line: new Date("2024-12-31").getTime() / 1000
};

// Token metadata for YES/NO shares
const tokenArgs = {
  yes_name: "BTC_100k_YES",
  yes_symbol: "BTC100K_Y",
  yes_uri: "https://example.com/yes-metadata.json",
  no_name: "BTC_100k_NO", 
  no_symbol: "BTC100K_N",
  no_uri: "https://example.com/no-metadata.json"
};
```

### Trading Shares

- **Buy Shares**: Purchase YES or NO outcome tokens
- **Sell Shares**: Liquidate position before market resolution
- **Price Discovery**: Automated through LMSR algorithm

### Market Resolution

- Markets resolve automatically at deadline
- Outcome determined by external oracle integration
- Winners receive proportional share of market pool

## 📊 Economic Model

### LMSR Pricing

The platform uses Logarithmic Market Scoring Rule for automated market making:

- **Cost Function**: `C = b * ln(e^(q1/b) + e^(q2/b))`
- **Share Calculation**: Delta-based pricing for continuous liquidity
- **Liquidity Parameter**: Configurable `b` parameter for market depth

### Fee Structure

- **Platform Fees**: Configurable percentage on trades
- **Treasury**: Accumulated fees distributed to platform treasury
- **Market Fees**: Additional fees for market creation and maintenance

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/prediction-markets.ts

# Run with verbose output
anchor test -- --nocapture
```

## 🚢 Deployment

### Local Development

```bash
# Start local validator
solana-test-validator

# Deploy to localnet
anchor deploy
```

### Devnet/Mainnet

```bash
# Configure for target network
solana config set --url <network-url>

# Deploy program
anchor deploy
```

## 📁 Project Structure

```
├── programs/                  # Solana programs
│   └── prediction-markets/    # Main prediction markets program
├── app/                       # Frontend application (future)
├── tests/                     # Integration tests
├── migrations/                # Deployment scripts
├── Anchor.toml               # Anchor configuration
├── Cargo.toml                # Rust dependencies
└── package.json              # Node.js dependencies
```
---

**Built with ❤️ on Solana**