# Worker Yew SPA

## Prerequisites

- Latest Rust toolchain with `wasm32-unknown-unknown` target
- npm
- `trunk` CLI ( installable by `cargo install trunk` )

In addition, `wasm-opt` is recommended to be installed.

See https://github.com/kana-rus/ohkami-yew-todo for a working example!

## Setup

```sh
npm create cloudflare ./path/to/project-dir -- --template https://github.com/kana-rus/ohkami-templates/worker_yew_spa
```
```sh
cd ./path/to/project-dir
```
```sh
npx wrangler login
```

If you push the project to your GitHub repo, **You should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```
```sh
trunk serve --watch src/ui --open
```

## Publish

```sh
npm run deploy
```

If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.
