pub type OrderId = u32;
pub type Dish = u8;
pub type Quantity = u8;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum OrderType {
    Submission = 0,
    Cancellation = 1,
    Cooked = 2,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    pub order_id: OrderId,
    pub dish: Option<Dish>,
    pub ordertype: OrderType,
    pub quantity: Quantity,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            order_id: 0,
            dish: None,
            ordertype: OrderType::Submission,
            quantity: 0,
        }
    }
}
