# Aomi app template

A small Aomi app you can deploy as is, then edit. The Rust app is at the repo
root. `web/` and `client/` are two ways to talk to it once it is deployed.

## What's inside

- `src/` holds the app `playground-example`, build `v1`. It is built with `aomi-sdk =6.0.0`.
  - `greet` and `about` are always available to the model.
  - `menu_search` and `order_total` belong to the `playground-example/cafe` skill
    in `src/skill/cafe.md`. The model sees them only after it activates the skill.
- `tests/tools.rs` tests every tool. It also checks the manifest and which skill owns each tool.
- `web/` is a Vite + React page with `<AomiWidget>` in direct mode.
- `client/` has Node scripts that use `@aomi-labs/client`. `pnpm --filter client tour` runs a scripted conversation.
- `.aomi/config.json` and `aomi.toml` are what Build reads to deploy.

The Rust build never touches the JS workspace.

Keep the `aomi-sdk = "=6.0.0"` pin in `Cargo.toml`. It must match the backend
runtime exactly. The deploy preflight rejects any other version.

## Test locally

```sh
cargo test
```

## Deploy

Pick one of the two paths below. Both end with a Project that deploys your own
copy of this repo. Start from Build or from a copy. Never connect this template
repo itself as a Project.

### Build

Open Aomi Build at [build.aomi.dev](https://build.aomi.dev). Choose "Start from
the template", or import a repo you made with "Use this template" on GitHub.
Build creates the Project, so you can skip `aomi-build project create`.

### CLI

Create your own repo from this one with "Use this template" on GitHub, then
clone it. Install the CLI. Version 6.0.2 or later is required.

```sh
cargo install aomi-sdk --version 6.0.2 --features cli --locked
```

Run these from your clone:

```sh
aomi-build login --backend https://api.aomi.dev
aomi-build project create
aomi-build deploy
```

`aomi-build login` opens GitHub in your browser to sign you in. It saves the
backend URL, so later commands reuse it.

`aomi-build project create` prints a GitHub authorization page for your repo.
Approve it within 10 minutes as a GitHub user with admin on the repo. The
Project is created when you approve. The command reads the repo from your
`origin` remote.

Useful flags for `project create`:

- `--repo owner/name` names the repo. It must match your `origin` remote.
- `--platform <name>` picks the platform. The default is `community`.
- `--build-url <url>` sets the Aomi Build URL. It is inferred from the backend.
- `--no-browser` prints the URL instead of opening a browser.

### Staging

Use the staging hosts for every step:

- Build: [build-staging.aomi.dev](https://build-staging.aomi.dev)
- CLI: `aomi-build login --backend https://api-staging.aomi.dev`
- Chat: `https://chat-staging.aomi.dev`, set through the env vars below

Deploying gives you an application id. Put it in the env files below.

## Widget

```sh
cp web/.env.example web/.env      # set VITE_AOMI_APPLICATION_ID
pnpm install
pnpm --filter web dev
```

The widget uses direct routing. Every message goes to this app.

## Client

```sh
cp client/.env.example client/.env      # set AOMI_APPLICATION_ID
pnpm install
pnpm --filter client tour               # the scripted conversation
pnpm --filter client ask "Greet Ada, excitedly"
```

Both print the host they talk to when they start. They use the production chat
host by default. For staging, set `VITE_AOMI_BASE_URL` in `web/.env` or
`AOMI_BASE_URL` in `client/.env` to `https://chat-staging.aomi.dev`.
