// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

impl Order {
    pub fn new(new_product_name: String, new_quantity: u16, new_unit_price: u16) -> Order{
        valid_product_name(new_product_name);
        valid_quantity(new_quantity);
        valid_unit_price(new_unit_price);

        Order {
            product_name: new_product_name,
            quantity: new_quantity,
            unit_price: new_unit_price,
        }
    }
    pub fn total(&self) -> u16 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn unit_price(self: &Self) -> &u16 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_name: String) {
        valid_product_name(new_name);
        self.product_name = new_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u16) -> () {
        valid_quantity(&new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(self: &mut Self, new_unit_price: u16) {
        valid_unit_price(&new_unit_price);
        self.unit_price = new_unit_price;
    }


}

fn valid_product_name(new_product_name: String) {
    if new_product_name.len() == 0 || new_product_name.len() > 300 {
        panic!("Invalid product name")
    }
}

fn valid_quantity(new_quantity: u16) {
    if new_quantity == 0 {
        panic!("Invalid quantity")
    }
}

fn valid_unit_price(new_unit_price: u16) {
    if new_unit_price == 0 {
        panic!("Invalid unit price")
    }
} 