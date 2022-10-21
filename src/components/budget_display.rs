use yew::prelude::*;

use crate::randomizer::budget::Transaction;
use crate::randomizer::Budget;

#[derive(PartialEq, Eq, Properties)]
pub struct BudgetDisplayProps {
    pub budget: Budget,
}

#[function_component]
pub fn BudgetDisplay(props: &BudgetDisplayProps) -> Html {
    let BudgetDisplayProps { budget } = props;

    let transactions_html = budget.debug_transactions
        .iter()
        .map(|transaction| match transaction {
            Transaction::Tool(refund, amount, name)
                | Transaction::Consumable(refund, amount, name)
                | Transaction::Weapon(refund, amount, name) => html! {
                <p>
                    if *refund {
                        <span class={classes!("has-text-danger")}>{&format!("Refund {name}")}</span>
                        <span>{&format!("-{amount}")}</span>
                    } else {
                        <span class={classes!("has-text-success")}>{&format!("Purchase {name}")}</span>
                        <span>{amount}</span>
                    }
                </p>
            },
            Transaction::Bullet(refund, amount, name) => html! {
                <p>
                    if *refund {
                        <span class={classes!("has-text-danger")}>{&format!("Refund {name} Ammo")}</span>
                        <span>{&format!("-{amount}")}</span>
                    } else {
                        <span class={classes!("has-text-success")}>{&format!("Purchase {name} Ammo")}</span>
                        <span>{amount}</span>
                    }
                </p>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("transactions-popup")}>
            <h3 class={classes!("is-size-3", "has-text-centered")}>
                {&format!("Initial Budget: {}", budget.initial_budget)}
            </h3>

            {transactions_html}

            <h5 class={classes!("is-size-5", "has-text-centered")}>
                {&format!("Amount Left Over: {}", budget.initial_budget - budget.total_cost)}
            </h5>

            <h3 class={classes!("is-size-3", "has-text-centered")}>
                {&format!("Total Cost: {}", budget.total_cost)}
            </h3>
        </div>
    }
}
