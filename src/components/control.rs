use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

use crate::entry::Entry;
use crate::message::Msg;

#[derive(Properties, Clone)]
pub struct Props {
    pub entry: Entry,
    pub on_edit: Callback<String>,
    pub on_remove: Callback<()>,
}

pub struct Control {
    memo_input_ref: NodeRef,
    link: ComponentLink<Self>,
    props: Props,
    value: String,
}

impl Component for Control {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.entry.description.clone();
        Control {
            memo_input_ref: NodeRef::default(),
            link,
            props,
            value,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::EmitEdit => {
                self.props.on_edit.emit(self.value.clone());
                self.focus();
            }
            Msg::EmitRemove => {
                self.props.on_remove.emit(());
                self.value = "".into();
                self.focus();
            }
            _ => {}
        }
        true
    }

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

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.focus();
        }
    }
}

impl Control {
    fn focus(&mut self) {
        if let Some(input) = self.memo_input_ref.cast::<InputElement>() {
            input.focus().expect("Failed to focus");
        }
    }
}
