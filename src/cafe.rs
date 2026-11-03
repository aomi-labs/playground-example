//! The Space Café skill: `menu_search` and `order_total` over a fixed menu.

use aomi_sdk::{DynAomiTool, DynToolCallCtx};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::Playground;

#[derive(Deserialize)]
struct Item {
    id: String,
    name: String,
    price_cents: u32,
    tags: Vec<String>,
}

fn menu() -> Vec<Item> {
    serde_json::from_str(include_str!("menu.json")).expect("menu.json is valid")
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MenuSearchArgs {
    /// Text to match against item names and tags, e.g. "coffee" or "mochi".
    query: String,
    /// Maximum items to return, 1 to 10. Defaults to 3.
    limit: Option<u32>,
}

pub struct MenuSearch;

impl DynAomiTool for MenuSearch {
    type App = Playground;
    type Args = MenuSearchArgs;

    const NAME: &'static str = "menu_search";
    const DESCRIPTION: &'static str =
        "Search the Space Café menu by name or tag. Returns items with id, name, price_cents and tags.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let limit = args.limit.unwrap_or(3);
        if !(1..=10).contains(&limit) {
            return Err("limit must be 1–10".into());
        }
        let query = args.query.trim().to_lowercase();
        let mut items = menu()
            .into_iter()
            .filter(|item| {
                item.name.to_lowercase().contains(&query)
                    || item.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        let items = items
            .into_iter()
            .take(limit as usize)
            .map(|item| {
                json!({
                    "id": item.id,
                    "name": item.name,
                    "price_cents": item.price_cents,
                    "tags": item.tags,
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({ "items": items }))
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct OrderLine {
    /// Menu item id exactly as returned by menu_search, e.g. "C-01".
    id: String,
    /// Quantity, 1 to 20.
    qty: u32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OrderTotalArgs {
    /// Items to order.
    items: Vec<OrderLine>,
}

pub struct OrderTotal;

impl DynAomiTool for OrderTotal {
    type App = Playground;
    type Args = OrderTotalArgs;

    const NAME: &'static str = "order_total";
    const DESCRIPTION: &'static str =
        "Price an order of Space Café items. Returns per-line subtotals and the total, all in cents.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let menu = menu();
        let mut total_cents = 0;
        let mut lines = Vec::with_capacity(args.items.len());
        for line in args.items {
            let item = menu
                .iter()
                .find(|item| item.id == line.id)
                .ok_or_else(|| format!("unknown_id: {}", line.id))?;
            if !(1..=20).contains(&line.qty) {
                return Err("qty must be 1–20".into());
            }
            let subtotal_cents = item.price_cents * line.qty;
            total_cents += subtotal_cents;
            lines.push(json!({
                "id": item.id,
                "name": item.name,
                "qty": line.qty,
                "subtotal_cents": subtotal_cents,
            }));
        }
        Ok(json!({ "lines": lines, "total_cents": total_cents }))
    }
}
