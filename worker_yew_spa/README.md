# Worker Yew SPA

## Prerequisites

- Recent Rust toolchain with `wasm32-unknown-unknown` target
- [`worker-build`](https://crates.io/crates/worker-build) ( run `cargo install worker-build` to install )
- npm
- `trunk` CLI ( run `cargo install trunk` to install )

In addition, `wasm-opt` ( pakcaged in [binaryen](https://github.com/WebAssembly/binaryen) ) is recommended to be installed for release build optimization.

See https://github.com/kanarus/ohkami-yew-todo for a working example!

## Setup

```sh
npm create cloudflare ＜project dir＞ -- --template https://github.com/ohkami-rs/ohkami-templates/worker_yew_spa
```
```sh
cd ＜project dir＞
```

If you push the project to your GitHub repo, **you should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```
```sh
trunk serve --watch src/ui --open
```

## Publish

```sh
npx wrangler login
```
```sh
npm run deploy
```

If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.
