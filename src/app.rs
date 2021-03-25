use log::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

use crate::components::container::Container;
use crate::components::control::Control;
use crate::constants::KEY;
use crate::entry::Entry;
use crate::message::Msg;
use crate::state::State;

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

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
}
