#![recursion_limit = "128"]

use yew::services::{ConsoleService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod stage;
use stage::Model as Stage;

pub struct Model;

pub enum Msg {
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("View");
        html! {
            <div class="root">
                <Stage />
            </div>
        }
    }
}