# Inventory Management System

A command-line inventory management system built with Rust, featuring product tracking, sales recording, and reporting functionality.

## Features

- Secure authentication system using environment variables
- Product management (Add, Edit, Delete, List)
- Inventory tracking
- Sales recording and tracking
- Purchase management
- Report generation
  - Inventory reports
  - Sales reports
  - Purchase reports

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/inventory-management
cd inventory-management
```

2. Create a `.env` file in the project root:
```bash
ADMIN_USERNAME=your_username
ADMIN_PASSWORD=your_password
```

3. Build the project:
```bash
cargo build --release
```

## Usage

Run the application:
```bash
cargo run
```

### Authentication

The system will prompt for username and password. These credentials should match those set in your environment variables.

### Main Menu Options

1. **Add Product**: Add a new product to inventory
2. **Edit Product**: Modify existing product details
3. **Delete Product**: Remove a product from inventory
4. **List Inventory**: View all products in stock
5. **Record Sale**: Record a product sale
6. **Record Purchase**: Record a product purchase
7. **Generate Reports**: Access various system reports
8. **Exit**: Close the application

### Reports

The system offers three types of reports:
- Inventory Report: Current stock levels
- Sales Report: Transaction history for sales
- Purchase Report: Transaction history for purchases

## Project Structure

```
src/
├── main.rs          # Application entry point and UI
```

<!-- ## Testing

Run the test suite:
```bash
cargo test
``` -->

## Security

- Credentials are stored as environment variables
- Password input is handled securely
- Authentication required for all operations

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Technical Details

### Product Structure
```rust
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}
```

### Sale Structure
```rust
struct Sale {
    product_id: u32,
    quantity: u32,
    sale_price: f64,
    profit: f64,
}
```

### Purchase Structure
```rust
struct Purchase {
    product_id: u32,
    quantity: u32,
    purchase_price: f64,
    total_cost: f64,
}
```

## Dependencies

- `dotenv`: Environment variable management
- Standard Rust libraries

Add to your `Cargo.toml`:
```toml
[dependencies]
dotenv = "0.15.0"
```

## Error Handling

The system includes error handling for:
- Invalid numeric inputs
- Product not found
- Insufficient stock
- Authentication failures

## Future Improvements

- Database integration
- User management system
- Export reports to CSV/PDF
- Web interface
- API endpoints
- Transaction logging
- Backup/restore functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Rust Community
- Contributors and testers

## Support

For support, email modupe775@gmail.com or open an issue in the repository.