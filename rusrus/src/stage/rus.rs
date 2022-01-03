use yew::{html, Component, Html, Properties, ShouldRender, ComponentLink};

extern crate js_sys;

#[derive(Debug)]
pub struct Model {
    props: Props,
}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Empty = 0,
    Green = 1,
    Blue = 2,
    Purple = 3,
    Red = 4,
    Yellow = 5,
}

impl Component for Model {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            props: props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.color == Color::Empty {
            html! {
                <div class="stage_cell"></div>
            }
        } else {
            let image_path = &format!("img/rus_{}.png", self.props.color as u32);
            html! {
                <div class="stage_cell">
                    <img src={ image_path.clone() } class="rus"/>
                </div>
            }
        }
    }
}

pub fn random_color() -> Color {
    let r = js_sys::Math::random();
    if r < 0.2 {
        Color::Green
    } else if r < 0.4 {
        Color::Blue
    } else if r < 0.6 {
        Color::Purple
    } else if r < 0.8 {
        Color::Red
    } else{
        Color::Yellow
    }
}
