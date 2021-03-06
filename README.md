# Pouch - Rust bindings for PouchDB

*PAY ATTENTION: NOT READY FOR USE*

Rust bindings to make PouchDB available for usage with Yew and other browser-based Rust frameworks.

Current features, and such in progress:

- [x] Create/open database
- [x] Close database
- [x] Get database info
- [ ] Create/update doc
- [ ] Delete doc
- [ ] Delete database

Quality aspects:

- [x] Example with Yew
- [x] ~~Regression tests with Node~~ (problems with pouchdb js exports)
- [x] ~~Regression tests with Chrome~~ (problems with pouchdb js exports)
- [ ] API documentation

## Test

*Deactivated currently due to problems with pouchdb module exports*

Launch all <code>wasm-bindgen-test</code> based tests with Node.js:

```shell
wasm-pack test --node
```

## Examples

### Yew

Take a look to the [Yew example](/examples/yew).

