# pouch - Rust bindings for PouchDB

Rust bindings to make PouchDB available for usage with Yew and other browser-based Rust frameworks.

## Examples

### Yew

A simple Yew app using Pouch.

Build it:

```shell
wasm-pack build examples/yew --target web --out-name wasm --out-dir ./html/static
```

And run it with [caddy](https://www.caddyserver.com):

```shell
caddy file-server --root examples/yew/html
```

