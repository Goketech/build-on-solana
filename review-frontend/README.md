# Web3 Restaurant Review DApp - README

Welcome to the **Web3 Restaurant Review DApp** project repository! This decentralized application (DApp) leverages blockchain technology to implement a review platform on the Solana network. Participants can create reviews on restaurants, and also see reviews given.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Smart Contracts](#smart-contracts)
- [Testing](#testing)
- [Frontend](#frontend)
- [Contributing](#contributing)
- [License](#license)

## Overview

The **Web3 Auction DApp** provides a user-friendly interface to participate in Solana-based reviews. This project ensures transparency and trust in the review process through the use of smart contracts. Users can create reviews, monitor reviews, and update their reviews.

## Features

- Browse current reviews.
- Real-time updates: Receive instant notifications for reviews.
- View Reviewss: Owners of reviews can update them.
- Solana Wallet Integration: Connect your Solana wallet (e.g., Solflare) to participate directly.

## Getting Started

Follow these steps to set up the project locally and start participating in web3 auctions.

### Prerequisites

1. Node.js: Ensure Node.js is installed. Download it from [nodejs.org](https://nodejs.org/).

### Installation

1. Clone the repository:

```bash
  git clone https://github.com/Goketech/build-on-solana.git
```

2. Navigate to the project directory:

```bash
  cd review-frontend
```

3. Install required npm packages:

```bash
 npm install
```

## Usage

1. Start the development server:

```bash
 npm run dev
```

2. Open your web browser and navigate to `http://localhost:3000` to access the DApp.

3. Connect your Solana wallet (e.g., Solflare) to the DApp.

4. Browse ongoing auctions, place bids, and monitor your auction activity.

## Smart Contracts

The smart contracts in this project facilitate the review process. They handle creation, transactions, and review process. These contracts are deployed on the Solana blockchain.

- `AuctionFactory.sol`: Responsible for creating new auctions.
- `Auction.sol`: Manages bidding and winner determination for individual auctions.

## Testing

Smart contract tests are located in the `test` folder. These tests ensure the correct functioning of the smart contract. To run the tests, follow these steps:

1. Open a terminal in the project directory.
2. Run the following command to execute the tests:

```bash
truffle test
```

This command will initiate the smart contract tests and display the results in the terminal.

![image](https://github.com/Rise-In/XXX-Bootcamp-FinalCase/assets/140731987/8dc52183-626c-4f39-9408-a37ba496a345)

## Frontend

The DApp frontend is built using modern web technologies including React.js. It provides an intuitive and interactive user interface for auction participation.

- **React.js**: Powers the DApp's user interface.
- **Web3.js**: The Ethereum JavaScript API for smart contract interaction.
- **MetaMask**: A popular Ethereum wallet browser extension for secure transactions.

## Contributing

Contributions to this project are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Make changes and test thoroughly.
4. Commit with clear and concise messages.
5. Push changes to your fork.
6. Submit a pull request describing your changes.

## License

This project is licensed under the [MIT License](LICENSE).

---

Thank you for your interest in the Web3 Auction DApp project! For questions or suggestions, reach out to us or open an issue on [GitHub](https://github.com/Goketech/build-on-solana). Happy bidding on the blockchain! 🚀