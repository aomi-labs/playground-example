use aomi_sdk::testing::{TestCtxBuilder, run_tool};
use aomi_sdk::{DynAomiApp, DynAomiTool, validate_app_skills_with_tools};
use playground_example::{About, BUILD, Greet, MenuSearch, OrderTotal, Playground};
use serde_json::{Value, json};

fn call<T: DynAomiTool<App = Playground>>(args: Value) -> Result<Value, String> {
    run_tool::<T>(&Playground, args, TestCtxBuilder::new(T::NAME).build()).map(|ret| ret.value)
}

#[test]
fn greet_says_hello() {
    assert_eq!(
        call::<Greet>(json!({ "name": "Ada" })).unwrap(),
        json!({ "greeting": "Hello, Ada." })
    );
    assert_eq!(
        call::<Greet>(json!({ "name": "Ada", "excited": true })).unwrap(),
        json!({ "greeting": "Hello, Ada!" })
    );
}

#[test]
fn greet_rejects_blank_name() {
    assert_eq!(call::<Greet>(json!({ "name": "  " })), Err("name is empty".into()));
}

#[test]
fn about_reports_build_and_session() {
    let ctx = TestCtxBuilder::new("about").session_id("s-1").build();
    let out = run_tool::<About>(&Playground, json!({}), ctx).unwrap().value;
    assert_eq!(out["app"], "playground-example");
    assert_eq!(out["build"], BUILD);
    assert_eq!(out["sdk"], aomi_sdk::AOMI_SDK_VERSION);
    assert_eq!(out["ffi"], aomi_sdk::AOMI_FFI_VERSION);
    assert_eq!(out["session_id"], "s-1");
}

#[test]
fn menu_search_matches_names_and_tags_sorted_by_id() {
    let out = call::<MenuSearch>(json!({ "query": "COFFEE" })).unwrap();
    assert_eq!(
        out,
        json!({ "items": [{ "id": "C-01", "name": "Nebula Latte", "price_cents": 450, "tags": ["coffee", "hot"] }] })
    );
    let out = call::<MenuSearch>(json!({ "query": "sweet", "limit": 10 })).unwrap();
    let ids: Vec<_> = out["items"].as_array().unwrap().iter().map(|i| i["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["C-03", "C-05", "C-06"]);
}

#[test]
fn menu_search_limits_and_empty_results() {
    let out = call::<MenuSearch>(json!({ "query": "" })).unwrap();
    assert_eq!(out["items"].as_array().unwrap().len(), 3);
    let out = call::<MenuSearch>(json!({ "query": "pizza" })).unwrap();
    assert_eq!(out, json!({ "items": [] }));
    assert_eq!(
        call::<MenuSearch>(json!({ "query": "a", "limit": 0 })),
        Err("limit must be 1–10".into())
    );
    assert_eq!(
        call::<MenuSearch>(json!({ "query": "a", "limit": 11 })),
        Err("limit must be 1–10".into())
    );
}

#[test]
fn order_total_prices_lines() {
    let out = call::<OrderTotal>(json!({ "items": [{ "id": "C-01", "qty": 2 }, { "id": "C-03", "qty": 1 }] })).unwrap();
    assert_eq!(out["total_cents"], 1180);
    assert_eq!(
        out["lines"],
        json!([
            { "id": "C-01", "name": "Nebula Latte", "qty": 2, "subtotal_cents": 900 },
            { "id": "C-03", "name": "Moon Mochi", "qty": 1, "subtotal_cents": 280 },
        ])
    );
}

#[test]
fn order_total_rejects_bad_input() {
    assert_eq!(
        call::<OrderTotal>(json!({ "items": [{ "id": "C-99", "qty": 1 }] })),
        Err("unknown_id: C-99".into())
    );
    for qty in [0, 21, 50] {
        assert_eq!(
            call::<OrderTotal>(json!({ "items": [{ "id": "C-02", "qty": qty }] })),
            Err("qty must be 1–20".into())
        );
    }
    let split = json!({ "items": [
        { "id": "C-02", "qty": 20 }, { "id": "C-02", "qty": 20 }, { "id": "C-02", "qty": 10 }
    ] });
    assert_eq!(call::<OrderTotal>(split), Err("qty must be 1–20".into()));
}

#[test]
fn manifest_has_four_tools_and_cafe_owns_two() {
    let manifest = Playground.manifest();
    assert_eq!(manifest.name, "playground-example");
    let mut names: Vec<_> = manifest.tools.iter().map(|t| t.name.as_str()).collect();
    names.sort();
    assert_eq!(names, ["about", "greet", "menu_search", "order_total"]);

    assert_eq!(manifest.skills.len(), 1);
    let skill = &manifest.skills[0];
    assert_eq!(skill.id, "playground-example/cafe");
    let mut owned = skill.injected_tools.clone();
    owned.sort();
    assert_eq!(owned, ["menu_search", "order_total"]);

    validate_app_skills_with_tools(&manifest.name, &manifest.skills, &manifest.tools)
        .expect("skills validate against tools");
}
