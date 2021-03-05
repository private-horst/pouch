import rust from "@wasm-tool/rollup-plugin-rust";
import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import json from "@rollup/plugin-json";
import nodePolyfills from 'rollup-plugin-node-polyfills';

export default {
    input: {
        "pouch-examples-yew": "Cargo.toml",
    },
    output: {
        name: "bundle",
        dir: "html/static",
        format: 'es'
    },
    plugins: [
        rust({
            serverPath: "/static/"
        }),
        json(),
        nodePolyfills(),
        nodeResolve({
            browser: true,
            jsnext: true,
            main: true
        }),
        commonjs({
            include: [ "node_modules/**" ]
        }),
    ],
};
