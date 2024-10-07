use crate::datatypes::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Waiter {
    pub menu: Vec<Dish>,
    pub orders: Vec<Order>,
}

#[allow(dead_code)]
impl Waiter {
    pub fn new(menu: Vec<Dish>) -> Waiter {
        Waiter {
            menu,
            orders: Vec::new(),
        }
    }

    pub fn update_menu(&mut self, menu: Vec<Dish>) {
        self.menu = menu;
    }

    pub fn receive_orders(&mut self, order_list: Vec<Order>) {
        for order in order_list {
            // Skip orders with zero quantity
            if order.quantity == 0 {
                continue;
            }

            // Skip orders for non-existent dishes
            if let Some(dish) = order.dish {
                if !self.menu.contains(&dish) {
                    continue;
                }
            }

            match order.ordertype {
                OrderType::Submission => {
                    if let Some(existing_order) = self
                        .orders
                        .iter_mut()
                        .find(|o| o.order_id == order.order_id)
                    {
                        existing_order.quantity += order.quantity;
                    } else {
                        self.orders.push(order);
                    }
                }
                OrderType::Cancellation => {
                    if let Some(existing_order) = self
                        .orders
                        .iter_mut()
                        .find(|o| o.order_id == order.order_id)
                    {
                        if existing_order.quantity > order.quantity {
                            existing_order.quantity -= order.quantity;
                        } else {
                            self.orders.retain(|o| o.order_id != order.order_id);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn serve_orders(&mut self, cooked_dish: Vec<Order>) {
        for order in cooked_dish {
            if let Some(existing_order) = self
                .orders
                .iter_mut()
                .find(|o| o.order_id == order.order_id)
            {
                if existing_order.quantity > order.quantity {
                    existing_order.quantity -= order.quantity;
                } else {
                    self.orders.retain(|o| o.order_id != order.order_id);
                }
            }
        }
    }

    pub fn report_remaining_orders(&mut self, levels: u8) -> Vec<(Dish, Quantity)> {
        let mut dish_quantities: std::collections::HashMap<Dish, Quantity> =
            std::collections::HashMap::new();

        for order in &self.orders {
            if let Some(dish) = order.dish {
                *dish_quantities.entry(dish).or_insert(0) += order.quantity;
            }
        }

        let mut sorted_dish_quantities: Vec<(Dish, Quantity)> =
            dish_quantities.into_iter().collect();
        sorted_dish_quantities.sort_by(|a, b| b.0.cmp(&a.0));

        let mut result = Vec::new();
        for i in 0..levels {
            if let Some(dish_quantity) = sorted_dish_quantities.get(i as usize) {
                result.push(*dish_quantity);
            } else {
                result.push((0, 0));
            }
        }

        result
    }
}
