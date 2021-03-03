# Pouch - Rust bindings for PouchDB (ALPHA)

NOTE: VERY EARLY STAGE, NOT READY FOR USE!

Rust bindings to make PouchDB available for usage with Yew and other browser-based Rust frameworks.

## Examples

### Yew

A simple Yew app using Pouch.

Build it:

```shell
wasm-pack build examples/yew --target web --out-name wasm --out-dir ./html/static
```

Test it:

```shell
wasm-pack test --node
```

And run it with [Caddy](https://caddyserver.com/):

```shell
caddy file-server --root examples/yew/html
```

