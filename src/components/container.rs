use yew::prelude::*;

use crate::message::Msg;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

pub struct Container {
    props: Props,
}

impl Component for Container {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Container { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            // Allow children to be re-rendered when `props.children` changes.
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="container">
                { self.props.children.clone() }
            </div>
        }
    }
}
