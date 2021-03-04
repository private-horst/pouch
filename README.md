# Pouch - Rust bindings for PouchDB (ALPHA)

NOTE: VERY EARLY STAGE, NOT READY FOR USE!

Rust bindings to make PouchDB available for usage with Yew and other browser-based Rust frameworks.

## Test

Launch all <code>wasm-bindgen-test</code> based tests with Node.js:

```shell
wasm-pack test --node
```

## Examples

### Yew

A simple Yew app using Pouch.

Build it ...

```shell
wasm-pack build examples/yew --target web --out-name wasm --out-dir ./html/static
```

... and run it with [Caddy](https://caddyserver.com/) ...

```shell
caddy run --config examples/yew/html/Caddyfile
```

on [localhost:8080](http://localhost:8080). Change the port in the <code>Caddyfile</code> if necessary.

