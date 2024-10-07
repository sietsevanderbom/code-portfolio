#[cfg(test)]
mod book_constructor_tests {
    use crate::{datatypes::*, waiter::Waiter};
    use rstest::*;

    #[rstest]
    fn test_update_menu() {
        let mut waiter = Waiter::new(vec![3, 4]);
        waiter.update_menu(vec![1, 5, 7]);

        for dish in [1, 5, 7] {
            assert!(waiter.menu.contains(&dish));
        }
    }

    #[rstest]
    fn test_report_orders() {
        let mut waiter = Waiter::new(vec![2, 3, 4]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(3),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
            Order {
                order_id: 2,
                dish: Some(4),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
            Order {
                order_id: 3,
                dish: Some(2),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
        ]);

        assert_eq!(waiter.report_remaining_orders(1), vec![(4, 1)]);
    }

    #[rstest]
    fn test_receive_orders() {
        let mut waiter = Waiter::new(vec![1, 3, 4]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(3),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
            Order {
                order_id: 2,
                dish: Some(4),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
        ]);

        waiter.receive_orders(vec![
            Order {
                order_id: 2,
                dish: None,
                ordertype: OrderType::Submission,
                quantity: 1,
            },
            Order {
                order_id: 3,
                dish: Some(1),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
        ]);

        assert_eq!(waiter.report_remaining_orders(2), vec![(4, 2), (3, 2)]);
    }

    #[rstest]
    fn test_receive_orders2() {
        let mut waiter = Waiter::new(vec![3, 4, 5]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(3),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
            Order {
                order_id: 2,
                dish: Some(4),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
        ]);

        waiter.receive_orders(vec![Order {
            order_id: 2,
            dish: None,
            ordertype: OrderType::Submission,
            quantity: 1,
        }]);

        waiter.receive_orders(vec![Order {
            order_id: 3,
            dish: Some(3),
            ordertype: OrderType::Submission,
            quantity: 1,
        }]);

        assert_eq!(
            waiter.report_remaining_orders(3),
            vec![(4, 3), (3, 3), (0, 0)]
        );
    }

    #[rstest]
    fn test_serve_orders() {
        let mut waiter = Waiter::new(vec![1, 3, 4]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(3),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
            Order {
                order_id: 2,
                dish: Some(4),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
        ]);

        waiter.receive_orders(vec![
            Order {
                order_id: 3,
                dish: Some(3),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
            Order {
                order_id: 4,
                dish: Some(4),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
        ]);

        waiter.serve_orders(vec![
            Order {
                order_id: 2,
                dish: Some(4),
                ordertype: OrderType::Cooked,
                quantity: 1,
            },
            Order {
                order_id: 4,
                dish: Some(4),
                ordertype: OrderType::Cooked,
                quantity: 1,
            },
        ]);

        assert_eq!(waiter.report_remaining_orders(2), vec![(4, 1), (3, 3)]);
    }

    #[rstest]
    fn test_update_menu_with_empty_list() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.update_menu(vec![]);
        assert!(waiter.menu.is_empty());
    }

    #[rstest]
    fn test_receive_orders_with_duplicate_order_ids() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(2),
                ordertype: OrderType::Submission,
                quantity: 1,
            },
            Order {
                order_id: 1,
                dish: Some(2),
                ordertype: OrderType::Submission,
                quantity: 2,
            },
        ]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 3)]);
    }

    #[rstest]
    fn test_cancel_orders() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![
            Order {
                order_id: 1,
                dish: Some(2),
                ordertype: OrderType::Submission,
                quantity: 3,
            },
            Order {
                order_id: 1,
                dish: None,
                ordertype: OrderType::Cancellation,
                quantity: 2,
            },
        ]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 1)]);
    }

    #[rstest]
    fn test_serve_orders_with_insufficient_quantity() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        waiter.serve_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Cooked,
            quantity: 2,
        }]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 1)]);
    }

    #[rstest]
    fn test_report_orders_with_more_levels_than_dishes() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        assert_eq!(
            waiter.report_remaining_orders(5),
            vec![(2, 3), (0, 0), (0, 0), (0, 0), (0, 0)]
        );
    }

    #[rstest]
    fn test_receive_orders_with_zero_quantity() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 0,
        }]);
        assert!(waiter.orders.is_empty());
    }

    #[rstest]
    fn test_serve_orders_with_zero_quantity() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        waiter.serve_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Cooked,
            quantity: 0,
        }]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 3)]);
    }

    #[rstest]
    fn test_cancel_orders_with_zero_quantity() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: None,
            ordertype: OrderType::Cancellation,
            quantity: 0,
        }]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 3)]);
    }

    #[rstest]
    fn test_update_menu_with_duplicate_dishes() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.update_menu(vec![2, 2, 3, 3, 4]);
        assert_eq!(waiter.menu, vec![2, 2, 3, 3, 4]);
    }

    #[rstest]
    fn test_receive_orders_with_non_existent_dish() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(4),
            ordertype: OrderType::Submission,
            quantity: 1,
        }]);
        assert!(waiter.orders.is_empty());
    }
    #[rstest]
    fn test_serve_orders_with_non_existent_order_id() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        waiter.serve_orders(vec![Order {
            order_id: 2,
            dish: Some(2),
            ordertype: OrderType::Cooked,
            quantity: 1,
        }]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, 3)]);
    }
    #[rstest]
    fn test_report_orders_with_zero_levels() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: 3,
        }]);
        assert_eq!(waiter.report_remaining_orders(0), vec![]);
    }

    #[rstest]
    fn test_receive_orders_with_maximum_quantity() {
        let mut waiter = Waiter::new(vec![1, 2, 3]);
        waiter.receive_orders(vec![Order {
            order_id: 1,
            dish: Some(2),
            ordertype: OrderType::Submission,
            quantity: u8::MAX,
        }]);
        assert_eq!(waiter.report_remaining_orders(1), vec![(2, u8::MAX)]);
    }
}
