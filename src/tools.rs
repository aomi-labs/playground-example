use aomi_sdk::{AOMI_FFI_VERSION, AOMI_SDK_VERSION, DynAomiTool, DynToolCallCtx};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{BUILD, Playground};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GreetArgs {
    /// Who to greet.
    name: String,
    /// Add an exclamation mark. Defaults to false.
    #[serde(default)]
    excited: bool,
}

pub struct Greet;

impl DynAomiTool for Greet {
    type App = Playground;
    type Args = GreetArgs;

    const NAME: &'static str = "greet";
    const DESCRIPTION: &'static str = "Greet someone by name, optionally with excitement.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let name = args.name.trim();
        if name.is_empty() {
            return Err("name is empty".into());
        }
        let punct = if args.excited { "!" } else { "." };
        Ok(json!({ "greeting": format!("Hello, {name}{punct}") }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AboutArgs {}

pub struct About;

impl DynAomiTool for About {
    type App = Playground;
    type Args = AboutArgs;

    const NAME: &'static str = "about";
    const DESCRIPTION: &'static str =
        "Report which build of this app is running: app name, build, SDK and FFI versions, and the session id.";

    fn run(_app: &Self::App, _args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        Ok(json!({
            "app": "playground-example",
            "build": BUILD,
            "sdk": AOMI_SDK_VERSION,
            "ffi": AOMI_FFI_VERSION,
            "session_id": ctx.session_id,
        }))
    }
}
