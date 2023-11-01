
use super::{Wbs, Plant};

/// SAP order type
#[derive(Debug)]
pub enum Order {
    /// Planned order
    PlannedOrder(OrderData),
    /// Production order (released)
    ProductionOrder(OrderData)
}

impl Order {
    /// creates a new Order from a given type and data
    pub fn new(order_type: &str, data: OrderData) -> Self {
        match order_type {
            "PP01" => Order::ProductionOrder(data),
            "PR"   => Order::PlannedOrder(data),
            _ => unreachable!()
        }
    }
}

/// Data for any given order
#[derive(Debug, Clone)]
pub struct OrderData {
    /// order number
    pub id: u32,
    /// piece mark
    pub mark: String,
    /// order quantity
    pub qty: u32,
    /// WBS element for the order
    pub wbs: Wbs,
    /// plant (Lancaster or Williamsport)
    pub plant: Plant
}

impl OrderData {
    /// Apply (reduce) the order quanity by a given amount.
    /// Amount being reduced must not be greater than the order quantity.
    pub fn apply_qty(&mut self, qty: u32) {
        if self.qty < qty {
            panic!("Cannot apply qty({}) greater than PlannedOrder({})", qty, self.qty);
        }

        self.qty -= qty;
    }
}
