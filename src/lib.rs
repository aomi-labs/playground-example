//! Aomi Playground: a starter app you can clone, edit, and redeploy.
//!
//! `greet` and `about` are always visible to the model. `menu_search` and
//! `order_total` belong to the `playground-example/cafe` skill, so the host only
//! exposes them after the model activates that skill. Add a tool by writing a
//! `DynAomiTool` impl and listing it below, then run `aomi-build deploy`.

mod cafe;
mod tools;

pub use cafe::{MenuSearch, OrderTotal};
pub use tools::{About, Greet};

use aomi_sdk::dyn_aomi_app;

/// Bump this when you ship a change so `about` shows which build is running.
pub const BUILD: &str = "v1";

/// App state. Keep it `Clone + Default`; the runtime constructs one per session.
#[derive(Clone, Default)]
pub struct Playground;

dyn_aomi_app!(
    app = Playground,
    name = "playground-example",
    version = "0.2.0",
    preamble = "You are the Aomi Playground agent. Greet people with greet. Use about when asked \
                which build is running. For anything about the Space Café menu or an order, \
                activate the playground-example/cafe skill first.",
    tools = [Greet, About],
    namespaces = [],
    skills = [{
        id: "playground-example/cafe",
        description: "Browse the Space Café menu and price an order",
        tags: ["menu", "order"],
        tools: [MenuSearch, OrderTotal],
        sections: { instructions: "skill/cafe.md" },
    }],
);
