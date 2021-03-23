# yew-setup-2021

My simple 'yew' app template for 2021.

[1. About](#1-about)  
[2. Dev + Build](#2-dev--build)  
[3. What I Did](#3-what-i-did)  
[4. Installed Packages: JS](#4-installed-packages--js)  
[5. Installed Packages: Rust](#5-installed-packages--rust)  
[6. LICENSE](#6-license)

&nbsp;

## 1. About

My simple 'yew' app template for 2021.  
Codes are basically taken from
[yew's official TODO app example](https://github.com/yewstack/yew-wasm-pack-template),
only, I made it simpler.

Some differences:
- No `entries` but a single `entry`
- No filtering list items
- Breaking into several components
- Separate files
- Calling parent callback from a child
- Webpack 5 latest configurations

&nbsp;

## 2. Dev + Build

### Dev

```
yarn start
```

### Prod

```
yarn build
```

&nbsp;

## 3. What I Did

Not much to mention, but you need to be careful with Webpack 5.  
You get the following error when compile:

```
ERROR in ./pkg/index_bg.wasm 1:0
Module parse failed: Unexpected character '' (1:0)
The module seem to be a WebAssembly module, but module is not flagged as WebAssembly module for webpack.
BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).
For files that transpile to WebAssembly, make sure to set the module type in the 'module.rules' section of the config (e. g. 'type: "webassembly/async"').
(Source code omitted for this binary file)
 @ ./pkg/index.js 1:0-40
 @ ./bootstrap.js 3:0-15
```

So, if you look into the parts in concern:

`pkg/index_bg.js`

```js
import * as wasm from './index_bg.wasm';
...
...
export function run_app() {
  wasm.run_app(); // `wasm` here is undefined
}
```

you will notice `wasm` is not properly imported.  
There, you need to set the following Webpack option:

`webpack.base.js`

```js
  experiments: {
    syncWebAssembly: true,
  }
```

&nbsp;

## 4. Installed Packages: JS

### All NPM Packages

```
yarn add --dev webpack webpack-cli webpack-dev-server css-loader style-loader postcss-loader autoprefixer webpack-merge clean-webpack-plugin copy-webpack-plugin license-webpack-plugin wasm-pack @wasm-tool/wasm-pack-plugin prettier pretty-quick
```

yarn remove http-server

### Webpack

- webpack
- webpack-cli
- webpack-dev-server
- css-loader
- style-loader
- postcss-loader
- autoprefixer
- webpack-merge
- clean-webpack-plugin
- copy-webpack-plugin
- license-webpack-plugin

### WASM Related

- wasm-pack
- @wasm-tool/wasm-pack-plugin

### Others

- prettier
- pretty-quick

&nbsp;

## 5. Installed Packages: Rust

Almost the same as
[yew's official TODO app](https://github.com/yewstack/yew-wasm-pack-template).  
[See Cargo.toml](Cargo.toml)

&nbsp;

## 6. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))

dev": "webpack-dev-server --mode development --open",
build": "webpack --mode production",
build:dev": "webpack --mode development",
