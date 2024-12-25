# Liquor Management System

The Liquor Management System is a decentralized application built on the Internet Computer platform, enabling efficient management of liquor inventory, sales, and supply chain events. It uses Rust and IC Canister Development Kit (CDK) for a robust and secure implementation.

## Features

1. **User Management**
   - Register and manage users with roles, points, and contact information.
   - Update user points and retrieve user details.

2. **Liquor Product Management**
   - Add, retrieve, and delete liquor products.
   - Update inventory levels through adjustments.
   - Query product details by ID or list all products.

3. **Sales Management**
   - Record sales transactions with detailed information.
   - List all sales records.

4. **Inventory Adjustments**
   - Log inventory changes and their reasons.
   - List all inventory adjustment records.

5. **Supply Chain Events**
   - Log supply chain events related to products.
   - List all supply chain event records.

6. **Data Persistence**
   - Stable storage for users, products, sales records, inventory adjustments, and supply chain events.

## Technical Details

- **Programming Language**: Rust
- **IC CDK**: Internet Computer Canister Development Kit
- **Storage**: Stable BTreeMap for data persistence
- **Serialization**: Serde and Candid for encoding/decoding

## Installation

### Prerequisites

- Install Rust: [https://www.rust-lang.org/](https://www.rust-lang.org/)
- Install the DFINITY Canister SDK: [https://internetcomputer.org/](https://internetcomputer.org/)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/ankodudu3/liquor-supply-dapp-rust.git
   cd liquor-supply-dapp-rust
   ````

2. Deploy to the local Internet Computer:
   ```bash
   dfx start --background
   dfx deploy
   ```

3. Interact with the canister:
   Use `dfx canister call` commands or integrate with a frontend application.

## Usage

### Add a User

```bash
dfx canister call liquor_management register_user '({ username = "JohnDoe"; role = "Manager"; contact_info = "johndoe@example.com"; })'
```

### Add a Liquor Product

```bash
dfx canister call liquor_management add_liquor_product '({
  name = "Premium Whiskey";
  user_id = 1;
  type = "Whiskey";
  brand = "Premium Spirits";
  alcohol_content = 40;
  batch_number = "B1234";
  bottle_size = "750ml";
  cost_price = 1500;
  retail_price = 2000;
  current_stock = 100;
  expiry_date = null;
})'
```

### Record a Sale

```bash
dfx canister call liquor_management record_sale '({
  liquor_product_id = 1;
  quantity = 2;
  sales_staff_id = 1;
  customer_age = 25;
})'
```

### Adjust Inventory

```bash
dfx canister call liquor_management log_inventory_adjustment '({
  liquor_product_id = 1;
  quantity_changed = 10;
  reason = "Restock";
  adjusted_by = "Admin";
})'
```

### View All Products

```bash
dfx canister call liquor_management list_all_products
```

## Data Structures

### User
```rust
struct User {
    id: u64,
    username: String,
    role: String,
    points: u64,
    contact_info: String,
}
```

### LiquorProduct
```rust
struct LiquorProduct {
    id: u64,
    user_id: u64,
    name: String,
    type: String,
    brand: String,
    alcohol_content: u64,
    batch_number: String,
    vintage_year: Option<String>,
    bottle_size: String,
    cost_price: u64,
    retail_price: u64,
    current_stock: u64,
    expiry_date: Option<String>,
}
```

### SaleRecord
```rust
struct SaleRecord {
    id: u64,
    liquor_product_id: u64,
    quantity: u64,
    total_price: u64,
    sale_date: u64,
    sales_staff_id: u64,
    customer_age: u64,
}
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- DFINITY Foundation for the Internet Computer platform
- Rust community for providing a robust programming language

