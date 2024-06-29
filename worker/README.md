# Cloudflare Workers template

## Prerequisites

- Rust toolchain of latest version with `wasm32-unknown-unknown` target
- npm

In addition, `wasm-opt` is recommended to be installed.

## Setup

```sh
npm create cloudflare ./path/to/project-dir -- --template https://github.com/ohkami-rs/ohkami-templates/worker
```
```sh
cd ./path/to/project-dir
```

and if you push the project to your GitHub repo, **You should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```

## Publish
```sh
npx wrangler login
```
```sh
npm run deploy
```
If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.
