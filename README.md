# yew-setup-2021

A simple [Yew](https://github.com/yewstack/yew) app boilerplate for 2021.

![screenshot](screenshot.png)

[View Demo](http://tokyo800.jp/mina/yew-setup-2021/)

[1. About](#1-about)  
[2. Dev + Build](#2-dev--build)  
[3. What I Did](#3-what-i-did)  
&nbsp; &nbsp; [3-1. Walk-through: JS-to-WASM Bootstrap](#3-1-walk-through-js-to-wasm-bootstrap)  
&nbsp; &nbsp; [3-2. Walk-through: Binding `App` to `#app`](#3-2-walk-through-binding-app-to-app)  
&nbsp; &nbsp; [3-3. Walk-through: Basic App Structure (`App`)](#3-3-walk-through-basic-app-structure-app)  
&nbsp; &nbsp; [3-4. Walk-through: Just A Wrapper (`Container`)](#3-4-walk-through-just-a-wrapper-container)  
&nbsp; &nbsp; [3-5. Walk-through: Event Handlers (`Control` + `Msg`)](#3-5-walk-through-event-handlers-control--msg)  
&nbsp; &nbsp; &nbsp; &nbsp; [(a) All Event Handlers (`Control`)](#a-all-event-handlers-control)  
&nbsp; &nbsp; &nbsp; &nbsp; [(b) Event Handlers and Messages (`Msg`)](#b-event-handlers-and-messages-msg)  
&nbsp; &nbsp; &nbsp; &nbsp; [(c) `Msg::EmitEdit` and `Msg::Edit(String)` (Update Button)](#c-msgemitedit-and-msgeditstring-update-button)  
&nbsp; &nbsp; &nbsp; &nbsp; [(d) `Msg::EmitRemove` and `Msg::Remove` (Remove Button)](#d-msgemitremove-and-msgremove-remove-button)  
&nbsp; &nbsp; &nbsp; &nbsp; [(e) `Msg::Update` (Input Form)](#e-msgupdate-input-form)  
&nbsp; &nbsp; [3-6. Walk-through: `State` + `Entry`](#3-6-walk-through-state--entry)  
&nbsp; &nbsp; [3-7. Others](#3-7-others)  
&nbsp; &nbsp; &nbsp; &nbsp; [(a) Webpack 5 Specific](#a-webpack-5-specific)  
&nbsp; &nbsp; &nbsp; &nbsp; [(b) Serving from Subdirectory](#b-serving-from-subdirectory)  
[4. Installed Packages: JS](#4-installed-packages--js)  
[5. Installed Packages: Rust](#5-installed-packages--rust)  
[6. LICENSE](#6-license)

&nbsp;

## 1. About

A simple boilerplate for [Yew](https://github.com/yewstack/yew) apps.
Codes are basically from its
_[official TODO app example](https://github.com/yewstack/yew-wasm-pack-template)_,
but simpler.

Some differences (from the official example):

- No `entries` but a single `entry` (for todos stored in state)
- No filtering on list items
- Breaking into several components
- Separate files (more modules)
- Calling parent callback from a child
- Bound to `#app`
- Webpack 5 latest configurations

For more, please read _[[3. What I Did]](#3-what-i-did)_ as it has a walk-through on the app structure.

&nbsp;

## 2. Dev + Build

### # dev

```
yarn start
```

### # prod

```
yarn build
```

&nbsp;

## 3. What I Did

Although the app is based on yew's
_[official TODO app example](https://github.com/yewstack/yew-wasm-pack-template)_,
it may be worth explaining how the app works...

### 3-1. Walk-through: JS-to-WASM Bootstrap

For any Yew apps, it runs
[wasm-pack](https://github.com/rustwasm/wasm-pack)
(and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) behind it)
on Webpack builds.  
Since Yew takes care of generating JS-to-WASM bootstrap files automatically,
you don't need to configure anything.
However, if you want to configure this manually, I wrote
[a friendly instruction](https://github.com/minagawah/perlin-experiment#3-what-i-did)
in the past which may help.

Now, take a look at `webpack.base.js`

[webpack.base.js](webpack.base.js):

```js
module.exports = {
  entry: './bootstrap.js',
  output: {
    filename: 'memo.js',
    path: path.resolve(__dirname, 'dist'),
    webassemblyModuleFilename: 'memo.wasm',
  },
  ...
  ...
  plugins: [
    ...
    ...
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './static',
          to: path.resolve(__dirname, 'dist'),
        },
      ],
    }),
    ...
    ...
```

`bootstrap.js` is specified for the build entry.
Upon building, it will emit all the bundles into `dist`.
As defined, the app is built into `dist/memo.js`.

Also, notice that I'm copying the contents inside `static`
to `dist` using `CopyWebpackPlugin`.

Once built, `dist` will look like this:

```
./dist
 ├── assets
 │  └── favicon.ico
 ├── index.html
 ├── memo.js
 ├── memo.wasm
 └── pkg_index_js.memo.js
```

There, `static/index.html` is also copied,
and it will become `dist/index.html`.

[static/index.html](static/index.html):

```html
<!DOCTYPE html>
<html>
  ... ... ...
  <script src="./memo.js"></script>
</html>
```

As the page loads, `dist/memo.js` will be loaded,
and it will load `dist/memo.wasm` asynchronously.

### 3-2. Walk-through: Binding `App` to `#app`

All the Rust codes are found in `src` directory.  
(except for [Cargo.toml](Cargo.toml))

```
./src
 ├── app.rs
 ├── components
 │   ├── container.rs
 │   ├── control.rs
 │   └── mod.rs
 ├── constants.rs
 ├── entry.rs
 ├── lib.rs
 ├── message.rs
 └── state.rs
```

As defined in `Cargo.toml`, it begins with `src/lib.rs`.

[src/lib.rs](src/lib.rs):

```rust
#![recursion_limit = "512"]

mod app;
pub mod components;
pub mod constants;
pub mod entry;
pub mod message;
pub mod state;

use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    if let Some(elem) = yew::utils::document().query_selector("#app").unwrap() {
        App::<app::App>::new().mount(elem);
        Ok(())
    } else {
        Err(JsValue::from("No element to bind"))
    }
}
```

Just like any other
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
apps, it will first run `run_app()`.
Then, `query_selector()` will look for `#app` in the page,
and `App` will be bound to the element.

Notice that it returns `Result<JsValue>` when mount fails.  
When this is returned,
it will automatically relay the error to `console.error`.

Also, check out that `src/lib.rs` declares modules:

```rust
pub mod components;
pub mod constants;
pub mod entry;
pub mod message;
pub mod state;
```

### 3-3. Walk-through: Basic App Structure (`App`)

When comparing `App` to that of Yew's official one,
you will notice it _has less lines_.
This is because I moved many of them into separate modules.

Before looking into modules, let's first look at `view()` in `App`.

[src/app.rs](src/app.rs):

```rust
    fn view(&self) -> Html {
        // Event handlers passed down to Control component.
        let on_edit_handler = self.link.callback(Msg::Edit);
        let on_remove_handler = self.link.callback(|_| Msg::Remove);

        html! {
            <Container>
                <Control
                    entry=self.state.entry.clone()
                    on_edit=on_edit_handler.clone()
                    on_remove=on_remove_handler.clone()
                />

                <div id="description">
                    { &self.state.entry.description }
                </div>

                <div id="footer">
                    <a href="https://github.com/minagawah/yew-setup-2021">
                        { "View Source" }
                    </a>
                </div>
            </Container>
        }
    }
```

From the above, you can tell:

- Everything is wrappedn in `<Container>`
- Passing event handlers to `<Control>` as props
- The event handlers emit `Msg::Xxx`
- Data will be stored to `self.state.entry`

Now, what do we have for `create()`?

```rust
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let entry = {
            if let Json(Ok(restored_entry)) = storage.restore(KEY) {
                restored_entry
            } else {
                Entry::new("")
            }
        };

        let state = State { entry };

        App {
            link,
            storage,
            state,
        }
    }
```

- It first checks in LocalStorage for previously saved data
- `self.state` is `State` struct
- `self.state.entry` is `Entry` struct

At this point, you are curious about the following structs:

- `Msg`
- `Container`
- `Control`
- `State`
- `Entry`

Look at how they are imported:

```rust
use crate::components::container::Container;
use crate::components::control::Control;
use crate::constants::KEY;
use crate::entry::Entry;
use crate::message::Msg;
use crate::state::State;
```

### 3-4. Walk-through: Just A Wrapper (`Container`)

Let's talk about _modules_ in Rust.

Say, you want to use `src/components/container.rs`,  
you first need `src/components/mod.rs`.

[src/components/mod.rs](src/components/mod.rs):

```rust
pub mod container;
pub mod control;
```

then, in `src/app.rs`, we have this.

[src/app.rs](src/app.rs):

```rust
use crate::components::container::Container;
```

That's how Rust modules work.  
Now, let's see what `Container` does.

Let's look at `view()`

[src/components/container.rs](src/components/container.rs):

```rust
    fn view(&self) -> Html {
        html! {
            <div id="container">
                { self.props.children.clone() }
            </div>
        }
    }
```

`Container` doesn't look like it's an exciting component
except for it wraps the given child with `<div></div>`...

However, `change()` has some interesting codes:

```rust
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            // Allow children to be re-rendered when `props.children` changes.
            self.props = props;
            true
        } else {
            false
        }
    }
```

For any Yew apps, unless it returns `true`, the component will not be updated.  
In order for `Container` to pick up changes in its children,
it has to compare the current props with the previous ones,
and return `true` when they have changes.

### 3-5. Walk-through: Event Handlers (`Control` + `Msg`)

#### (a) All Event Handlers (`Control`)

Now, let's take a look at `Control`.  
I call this module, _Control_, because it has forms and buttons.

Looks like there are a lot going on in `Control`.

Let's see what we have in `view()`

[src/components/control.rs](src/components/control.rs):

```rust
    fn view(&self) -> Html {
        let inputting = self.link.callback(|e: InputData| Msg::Update(e.value));
        let pressing = self.link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::EmitEdit
            } else {
                Msg::Nope
            }
        });
        let editing = self.link.callback(|_| Msg::EmitEdit);
        let removing = self.link.callback(|_| Msg::EmitRemove);

        let mut entry_style = "entry".to_string();

        // A bit darker color when editing.
        if self.props.entry.editing {
            entry_style.push_str(" entry-in-progress");
        }

        html! {
            <div id="control">
                <input
                    ref=self.memo_input_ref.clone()
                    value=&self.value
                    class=entry_style
                    oninput=inputting
                    onkeypress=pressing
                />

                <div id="buttons">
                    <button class="btn btn-update" onclick=editing>
                        { "Update" }
                    </button>

                    <button class="btn btn-remove" onclick=removing>
                        { "Remove" }
                    </button>
                </div>
            </div>
        }
    }
```

It has input forms and buttons, and they have corresponding event handlers.

Also, we have `memo_input_ref` attached to an input form.  
This is a _ref_ (which is `NodeRef`) for the element
so that I can later _focus_ on the input form.

In the next section, let's look at these event handlers.

#### (b) Event Handlers and Messages (`Msg`)

So, we have several event handlers defined in `Control`.

[src/components/control.rs](src/components/control.rs):

```rust
        let inputting = self.link.callback(|e: InputData| Msg::Update(e.value));
        let pressing = self.link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::EmitEdit
            } else {
                Msg::Nope
            }
        });
        let editing = self.link.callback(|_| Msg::EmitEdit);
        let removing = self.link.callback(|_| Msg::EmitRemove);
```

They all emit messages, or `Msg`.  
This is how they are defined.

[src/message.rs](src/message.rs):

```rust
pub enum Msg {
    Update(String),
    EmitEdit,
    Edit(String),
    EmitRemove,
    Remove,
    Nope,
}
```

#### (c) `Msg::EmitEdit` and `Msg::Edit(String)` (Update Button)

What does _Update_ button do?  
When _Update_ is clicked, `editing()` will run.  
Then, in `editing()`, it will emit `Msg::EmitEdit`.

What's in `Msg::EmitEdit`?

[src/components/control.rs](src/components/control.rs):

```rust
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::EmitEdit => {
                // Calling the parent's callback.
                self.props.on_edit.emit(self.value.clone());
                self.focus();
            }
            Msg::EmitRemove => {
                // Calling the parent's callback.
                self.props.on_remove.emit(());
                self.value = "".into();
                self.focus();
            }
            _ => {}
        }
        true
    }
```

So, it calls `self.props.on_edit.emit()`.  
Looking at `self.props.on_edit.emit()`,
you will notice it calls `on_edit_handler()`.  
This is a prop handed down from `App`.

[src/app.rs](src/app.rs):

```rust
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(_) => {
                self.state.entry.editing = true;
            }
            Msg::Edit(val) => {
                info!("val: {}", val);
                self.state.entry.description = val;
                self.state.entry.editing = false;
            }
            Msg::Remove => {
                self.state.reset();
                self.state.entry.editing = false;
            }
            _ => {}
        }
        self.storage.store(KEY, Json(&self.state.entry));
        true
    }

    fn view(&self) -> Html {
        // Event handlers passed down to Control component.
        let on_edit_handler = self.link.callback(Msg::Edit);
        let on_remove_handler = self.link.callback(|_| Msg::Remove);

        html! {
            <Container>
                <Control
                    entry=self.state.entry.clone()
                    on_edit=on_edit_handler.clone()
                    on_remove=on_remove_handler.clone()
                />

                <div id="description">
                    { &self.state.entry.description }
                </div>

                <div id="footer">
                    <a href="https://github.com/minagawah/yew-setup-2021">
                        { "View Source" }
                    </a>
                </div>
            </Container>
        }
    }
```

What about `on_edit_handler()`?  
What does it do?  
It emits `Msg::Edit(String)`.  
And, that's how the story ends...

```rust
let on_edit_handler = self.link.callback(Msg::Edit);
```

OK. Let's recap the steps covered:

1. Clicking _Update_ will first emit `Msg::EmitEdit`
2. (in `Msg::EmitEdit`) it will call `self.props.on_edit.emit()`, and
3. will finally emit `Msg::Edit(String)`

Get it?  
This is what happens when user clicks _Update_.

#### (d) `Msg::EmitRemove` and `Msg::Remove` (Remove Button)

What about `Msg::EmitRemove` and `Msg::Remove`?  
Well, just like clicking _Update_ button,
they will be emitted when _Remove_ button is clicked.
The steps are about the same,
except they don't carry around the value
for they simple aim to delete it.

#### (e) `Msg::Update` (Input Form)

Alright.  
Now, we understand how buttons work...

So, what is `Msg::Update`?

Let's look at the corresponding handler.

[src/components/control.rs](src/components/control.rs):

```rust
        let inputting = self.link.callback(|e: InputData| Msg::Update(e.value));
        ...
        ...
                <input
                    ref=self.memo_input_ref.clone()
                    value=&self.value
                    class=entry_style
                    oninput=inputting
                    onkeypress=pressing
                />
```

and the corresponding job for `Msg::Update(String)`:

```rust
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            ...
            ...
```

Here is what happens:

1. As users type into the input form, it continuously calls `inputting()`
2. In `inputting()`, it emits `Msg::Update`
3. In `Msg::Update`, it simply sets the value entered to `self.value`

`self.value`, when user clicks _Update_ button,
will eventually be passed down to `Msg::EmitEdit`.

### 3-6. Walk-through: `State` + `Entry`

Alright. How about `Msg::Edit(String)`?  
What does it do?

[src/app.rs](src/app.rs):

```rust
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(_) => {
                self.state.entry.editing = true;
            }
            Msg::Edit(val) => {
                info!("val: {}", val);
                self.state.entry.description = val;
                self.state.entry.editing = false;
            }
            Msg::Remove => {
                self.state.reset();
                self.state.entry.editing = false;
            }
            _ => {}
        }
        self.storage.store(KEY, Json(&self.state.entry));
        true
    }
```

Essentially, the given value is eventually set to `self.state.entry.description`:

```rust
                self.state.entry.description = val;
                self.state.entry.editing = false;
```

So, we are setting the value to `Entry` (in `State`).

Let's first look at `Entry`.

[src/entry.rs](src/entry.rs):

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub description: String,
    pub editing: bool,
}

impl Entry {
    pub fn new(description: &str) -> Self {
        Entry {
            description: description.into(),
            editing: false,
        }
    }
}
```

and `State`

[src/state.rs](src/state.rs):

```rust
use crate::entry::Entry;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub entry: Entry,
}

impl State {
    pub fn reset(&mut self) {
        self.entry = Entry::new("");
    }
}
```

Aren't they obvious?  
They look pretty straight forward to me...

### 3-7. Others

Some other configurations worth watching out for.

#### (a) Webpack 5 Specific

May not be a big deal, but you will get the following error when you use Webpack 5:

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

Here is the part in concern:

`pkg/index_bg.js`:

```js
import * as wasm from './index_bg.wasm';
...
...
export function run_app() {
  wasm.run_app(); // `wasm` here is undefined
}
```

Notice `wasm` here is not properly imported.  
For this, you need the following Webpack option:

`webpack.base.js`:

```js
  experiments: {
    syncWebAssembly: true,
  }
```

#### (b) Serving from Subdirectory

For `production`, I serve the app in _subdirectory_.
Hence, I need `output.publicPath` in `webpack.prod.js`.

[webpack.prod.js](webpack.prod.js):

```js
  output: {
    publicPath: '/mina/yew-setup-2021/',
  },
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
See [Cargo.toml](Cargo.toml)

&nbsp;

## 6. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))
