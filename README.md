# pouch - Rust bindings for PouchDB (EARLY ALPHA)

NOTE: VERY EARLY STAGE, NOT READY FOR USE YET!

Rust bindings to make PouchDB available for usage with Yew and other browser-based Rust frameworks.

## Examples

### Yew

A simple Yew app using Pouch.

Build it:

```shell
wasm-pack build examples/yew --target web --out-name wasm --out-dir ./html/static
```

And run it with [Caddy](https://caddyserver.com/):

```shell
caddy file-server --root examples/yew/html
```

