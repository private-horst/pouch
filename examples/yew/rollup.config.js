import rust from "@wasm-tool/rollup-plugin-rust";
import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import json from "@rollup/plugin-json";

export default {
    input: {
        "pouch-examples-yew": "Cargo.toml",
    },
    output: {
        name: "bundle",
        dir: "html/static",
        format: 'iife'
    },
    plugins: [
        rust({
            serverPath: "/static/"
        }),
        json(),
        nodeResolve({
            jsnext: true,
            main: true,
            preferBuiltins: true
        }),
        commonjs({
            include: [ "node_modules/**" ]
        }),
    ],
};
