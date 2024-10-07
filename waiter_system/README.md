# Rust Assessment

There is no time limit on submission, but we record the time it takes. Our estimate is around one to two hours.
The evaluation criteria are understanding of the scenario, time complexity, and consideration of edge cases.
Under the src/ folder, there are a few test cases in test_waiter.rs, which you can run with the `cargo test` command in the terminal.

Feel free to add additional methods or functions that you may need, change the variable names/structure of the code as long as it maintains the below requirements, and add other dependencies if needed.

## Introduction

You are going to code the Waiter system in `waiter.rs`, which will be used by a stupid chef who always wants to serve the most expensive dish first. For convenience, each dish is written as price (unsigned int), so there is no need for additional conversion between dish name and their price. The menu, which is the list of dishes, will be received by the Waiter when a new object is created.

## Methods required for Waiter

Follow waiter.rs for detailed input and output types.

- `update_menu` : the capricious chef wants to update the menu in the waiter system.

- `receive_orders` : receives the list of orders from customers and records it on the system's order book. There are two types of orders, which are SUBMISSION and CANCELLATION.

    SUBMISSION submits the new order or adds the order to add quantities on the previously submitted order. New submission will have both OrderId and dish price (which dish the order wants), while the adding quantity only has OrderId, which can track the previously submitted order.

    CANCELLATION decreases partially or entirely the quantity of previously submitted order. These CANCELLATION only has OrderId. Waiter can adjust or remove the previously submitted order through OrderId.

- `serve_orders` : receives the list of dishes cooked by chef and update the system's order book. The input will share the same Order struct as above `receive_orders`, but will consist with only one type of ordertype, COOKED. The dish price will not be provided.

- `report_remaining_orders`: returns the list of tuples with a format of (Dish, Remaining quantity). Assume you need to tell the chef how many orders (Quantity-wise) are left at each dish. The chef might want only one or two dishes of his priority, so the function takes an argument `levels` which denotes the number of the dishes that he wants to know their remaining quantities.

## Data Structures

Refer to datatypes.rs

- OrderId: u32, unique value for each Order.
- Dish: u8, assume each value represent a different dish
- Quantity: u8, denotes the quantities of each dish
- OrderType: type of orders
  - SUBMISSION: customer made a new order
  - CANCELLATION: cancel or adjust the quantity of the order that the system previously received

- Order: Each order represents the different dishes and quantities that the customer wants. So, the customer provides the list of these Order objects, which is the input of the `receive_orders` method. All the Order in this list have unique OrderId.
