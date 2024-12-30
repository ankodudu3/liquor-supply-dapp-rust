#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Memory type aliases
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    role: String,
    points: u64,
    contact_info: String,
}

// LiquorProduct struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LiquorProduct {
    id: u64,
    user_id: u64,
    name: String,
    product_type: String,
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

// SaleRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SaleRecord {
    id: u64,
    liquor_product_id: u64,
    quantity: u64,
    total_price: u64,
    sale_date: u64,
    sales_staff_id: u64,
    customer_age: u64,
}

// InventoryAdjustment struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InventoryAdjustment {
    id: u64,
    liquor_product_id: u64,
    quantity_changed: i64, // Allows both positive and negative adjustments
    reason: String,
    adjusted_by: u64,
    adjustment_date: u64,
}

// SupplyChainEvent struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SupplyChainEvent {
    id: u64,
    liquor_product_id: u64,
    event_type: String,
    location: String,
    quantity: u64,
    date: u64,
    participant_id: u64,
}

// Message enum
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    InsufficientStock(String),
    AgeRestriction(String),
}

// Implementing Storable and BoundedStorable for structs
impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for LiquorProduct {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for LiquorProduct {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SaleRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for SaleRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for InventoryAdjustment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for InventoryAdjustment {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SupplyChainEvent {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for SupplyChainEvent {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management using thread_local
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static LIQUOR_PRODUCTS_STORAGE: RefCell<StableBTreeMap<u64, LiquorProduct, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static SALE_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, SaleRecord, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    static INVENTORY_ADJUSTMENTS_STORAGE: RefCell<StableBTreeMap<u64, InventoryAdjustment, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );

    static SUPPLY_CHAIN_EVENTS_STORAGE: RefCell<StableBTreeMap<u64, SupplyChainEvent, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
        )
    );
}

// Function to generate a unique ID
fn generate_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).expect("Counter increment failed");
        current_value
    })
}

// Input validation function
fn validate_string_field(field: &str, field_name: &str) -> Result<(), Message> {
    if field.trim().is_empty() {
        return Err(Message::InvalidPayload(format!("{} cannot be empty", field_name)));
    }
    Ok(())
}

// Function to register a user
#[ic_cdk::update]
fn register_user(username: String, role: String, contact_info: String) -> Result<User, Message> {
    validate_string_field(&username, "Username")?;
    validate_string_field(&role, "Role")?;
    validate_string_field(&contact_info, "Contact Info")?;

    let user_id = generate_id();

    let user = User {
        id: user_id,
        username: username.clone(),
        role,
        points: 0,
        contact_info,
    };

    USERS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(user_id, user.clone());
    });

    Ok(user)
}

// Additional Function: Check stock availability
#[ic_cdk::query]
fn check_stock_availability(product_id: u64) -> Result<u64, Message> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&product_id)
            .map(|product| product.current_stock)
            .ok_or(Message::NotFound("Product not found".to_string()))
    })
}

// Additional Function: List products below stock threshold
#[ic_cdk::query]
fn list_low_stock_products(threshold: u64) -> Vec<LiquorProduct> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, product)| product.current_stock < threshold)
            .map(|(_, product)| product.clone())
            .collect()
    })
}

// Additional Function: Generate sales report
#[ic_cdk::query]
fn generate_sales_report() -> Vec<(String, u64)> {
    SALE_RECORDS_STORAGE.with(|sales_storage| {
        let sales = sales_storage.borrow();
        LIQUOR_PRODUCTS_STORAGE.with(|products_storage| {
            let products = products_storage.borrow();
            sales
                .iter()
                .filter_map(|(_, sale)| {
                    products
                        .get(&sale.liquor_product_id)
                        .map(|product| (product.name.clone(), sale.total_price))
                })
                .collect()
        })
    })
}

// Function to search for a user by username
#[ic_cdk::query]
fn search_user_by_username(username: String) -> Result<User, Message> {
    USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.username == username)
            .map(|(_, user)| user.clone())
            .ok_or(Message::NotFound("User not found".to_string()))
    })
}

// Function to add a liquor product
#[ic_cdk::update]
fn add_liquor_product(payload: LiquorProduct) -> Result<LiquorProduct, Message> {
    if payload.name.is_empty() {
        return Err(Message::InvalidPayload(
            "Product name is required".to_string(),
        ));
    }

    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(payload.id, payload.clone());
    });

    Ok(payload)
}

// Function to search products by name
#[ic_cdk::query]
fn search_products_by_name(name: String) -> Vec<LiquorProduct> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, product)| product.name.contains(&name))
            .map(|(_, product)| product.clone())
            .collect()
    })
}

// Function to update stock for a product
#[ic_cdk::update]
fn update_stock(product_id: u64, new_stock: u64) -> Result<LiquorProduct, Message> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(product) = storage.get(&product_id) {
            let mut updated_product = product.clone();
            updated_product.current_stock = new_stock;
            storage.insert(product_id, updated_product.clone());
            Ok(updated_product)
        } else {
            Err(Message::NotFound("Product not found".to_string()))
        }
    })
}

// Function to list all liquor products
#[ic_cdk::query]
fn list_all_products() -> Vec<LiquorProduct> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, product)| product.clone())
            .collect()
    })
}

// Function to sell liquor product
#[ic_cdk::update]
fn sell_liquor_product(payload: SaleRecord) -> Result<SaleRecord, Message> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(product) = storage.get(&payload.liquor_product_id) {
            let mut updated_product = product.clone();
            
            if updated_product.current_stock < payload.quantity {
                return Err(Message::InsufficientStock(
                    "Insufficient stock to sell the product".to_string(),
                ));
            }
            if payload.customer_age < 21 {
                return Err(Message::AgeRestriction(
                    "Customer must be at least 21 years old to purchase".to_string(),
                ));
            }

            updated_product.current_stock -= payload.quantity;
            storage.insert(updated_product.id, updated_product);

            SALE_RECORDS_STORAGE.with(|sales| {
                sales.borrow_mut().insert(payload.id, payload.clone());
            });

            Ok(payload)
        } else {
            Err(Message::NotFound("Product not found".to_string()))
        }
    })
}

// Function to list all sale records
#[ic_cdk::query]
fn list_all_sales() -> Vec<SaleRecord> {
    SALE_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect()
    })
}

// Function to log inventory adjustment
#[ic_cdk::update]
fn log_inventory_adjustment(payload: InventoryAdjustment) -> Result<InventoryAdjustment, Message> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(product) = storage.get(&payload.liquor_product_id) {
            let mut updated_product = product.clone();
            
            updated_product.current_stock = 
                ((updated_product.current_stock as i64) + payload.quantity_changed as i64) as u64;
            storage.insert(updated_product.id, updated_product);

            INVENTORY_ADJUSTMENTS_STORAGE.with(|adjustments| {
                adjustments.borrow_mut().insert(payload.id, payload.clone());
            });

            Ok(payload)
        } else {
            Err(Message::NotFound("Product not found".to_string()))
        }
    })
}
// Function to list all inventory adjustments
#[ic_cdk::query]
fn list_all_inventory_adjustments() -> Vec<InventoryAdjustment> {
    INVENTORY_ADJUSTMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, adjustment)| adjustment.clone())
            .collect()
    })
}

// Function to log a supply chain event
#[ic_cdk::update]
fn log_supply_chain_event(payload: SupplyChainEvent) -> Result<SupplyChainEvent, Message> {
    SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(payload.id, payload.clone());
    });
    Ok(payload)
}

// Function to list all supply chain events
#[ic_cdk::query]
fn list_all_supply_chain_events() -> Vec<SupplyChainEvent> {
    SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, event)| event.clone())
            .collect()
    })
}

// Function to get a user by ID
#[ic_cdk::query]
fn get_user_by_id(user_id: u64) -> Result<User, Message> {
    USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&user_id)
            .map(|user| user.clone())
            .ok_or(Message::NotFound("User not found".to_string()))
    })
}
// Function to update user points
#[ic_cdk::update]
fn update_user_points(user_id: u64, points: u64) -> Result<User, Message> {
    USERS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(user) = storage.get(&user_id) {
            let mut updated_user = user.clone();
            updated_user.points += points;
            storage.insert(user_id, updated_user.clone());
            Ok(updated_user)
        } else {
            Err(Message::NotFound("User not found".to_string()))
        }
    })
}

// Function to delete a liquor product
#[ic_cdk::update]
fn delete_liquor_product(product_id: u64) -> Result<String, Message> {
    LIQUOR_PRODUCTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&product_id).is_some() {
            Ok("Product deleted successfully".to_string())
        } else {
            Err(Message::NotFound("Product not found".to_string()))
        }
    })
}

// Function to get all users
#[ic_cdk::query]
fn list_all_users() -> Vec<User> {
    USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect()
    })
}

// Exporting the candid interface
ic_cdk::export_candid!();
