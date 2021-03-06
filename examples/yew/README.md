# Pouch example with Yew

A simple Yew app using Pouch.

Currently integrated Pouch features:

- [x] Create/open database
- [ ] Get database info
- [ ] Create doc
- [ ] Change doc
- [ ] Delete doc
- [ ] Close database
- [ ] Delete database

Change into the example directory, ...
```shell
cd examples/yew
```

... install all npm dependencies, ...
```shell
npm i
```

... build and bundle the package, ...
```shell
rollup -c
```

... and run it with [Caddy](https://caddyserver.com/) ...
```shell
caddy run
```

... on [localhost:8080](http://localhost:8080). Change the port in the <code>Caddyfile</code> if necessary.

