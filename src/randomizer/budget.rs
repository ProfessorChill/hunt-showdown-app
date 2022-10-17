// checked_add_signed is experimental, so until it's pushed to the stable branch we can just use
// bool as refund (true) or purchase (false) and u16 as the amount.
#[derive(Debug, Clone)]
pub enum Transaction {
    Bullet(bool, u16),
    Tool(bool, u16),
    Consumable(bool, u16),
    Weapon(bool, u16),
}

pub enum TransactionResult {
    InsufficientFunds,
    ErrorBudgeting,
}

#[derive(Debug, Clone)]
pub struct Budget {
    debug_transactions: Vec<Transaction>,
    pub total_cost: u16,
    pub initial_budget: u16,
    pub tools_budget: u16,
    pub consumables_budget: u16,
    pub weapons_budget: u16,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            debug_transactions: vec![],
            total_cost: 0,
            initial_budget: u16::MAX,
            tools_budget: u16::MAX,
            consumables_budget: u16::MAX,
            weapons_budget: u16::MAX,
        }
    }
}

fn reset_budget(budget: &mut Budget) -> Result<(), TransactionResult> {
    budget.total_cost = 0;
    budget.tools_budget = budget.initial_budget / 6;
    budget.consumables_budget = budget.initial_budget / 4;
    budget.debug_transactions.clear();

    match budget.initial_budget.checked_sub(
        budget
            .tools_budget
            .saturating_add(budget.consumables_budget),
    ) {
        Some(val) => {
            budget.weapons_budget = val;
            Ok(())
        }
        None => Err(TransactionResult::ErrorBudgeting),
    }
}

pub fn transfer_weapons_to_tools(budget: &mut Budget) {
    budget.tools_budget = budget.tools_budget.saturating_add(budget.weapons_budget);
}

pub fn transfer_tools_to_consumables(budget: &mut Budget) {
    budget.consumables_budget = budget
        .consumables_budget
        .saturating_add(budget.tools_budget);
}

pub fn set_budget(budget: &mut Budget, new_budget: u16) -> Result<(), TransactionResult> {
    budget.initial_budget = new_budget;

    reset_budget(budget)
}

pub fn process_transaction(
    budget: &mut Budget,
    transaction_type: Transaction,
) -> Result<(), TransactionResult> {
    budget.debug_transactions.push(transaction_type.clone());

    match transaction_type {
        Transaction::Bullet(refund, amount) | Transaction::Weapon(refund, amount) => {
            if refund {
                // Since it's not an error to have too much money we just set it to max if we
                // overflow.
                budget.weapons_budget = budget.weapons_budget.saturating_add(amount);
                budget.total_cost = budget.total_cost.saturating_sub(amount);

                Ok(())
            } else {
                match budget.weapons_budget.checked_sub(amount) {
                    Some(val) => {
                        budget.weapons_budget = val;
                        budget.total_cost = budget.total_cost.saturating_add(amount);

                        Ok(())
                    }
                    // We underflowed, don't do the calculation, not enough money.
                    None => Err(TransactionResult::InsufficientFunds),
                }
            }
        }
        Transaction::Consumable(refund, amount) => {
            if refund {
                budget.consumables_budget = budget.consumables_budget.saturating_add(amount);
                budget.total_cost = budget.total_cost.saturating_sub(amount);

                Ok(())
            } else {
                match budget.consumables_budget.checked_sub(amount) {
                    Some(val) => {
                        budget.consumables_budget = val;
                        budget.total_cost = budget.total_cost.saturating_add(amount);

                        Ok(())
                    }
                    None => Err(TransactionResult::InsufficientFunds),
                }
            }
        }
        Transaction::Tool(refund, amount) => {
            if refund {
                budget.tools_budget = budget.tools_budget.saturating_sub(amount);
                budget.total_cost = budget.total_cost.saturating_sub(amount);

                Ok(())
            } else {
                match budget.tools_budget.checked_sub(amount) {
                    Some(val) => {
                        budget.tools_budget = val;
                        budget.total_cost = budget.total_cost.saturating_add(amount);

                        Ok(())
                    }
                    None => Err(TransactionResult::InsufficientFunds),
                }
            }
        }
    }
}
