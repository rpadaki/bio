use stylist::css;
use yew::prelude::*;

enum Msg {
    TractorClicked,
}

struct Main {
    link: ComponentLink<Self>,
    translate: f64,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            translate: -5.0,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TractorClicked => {
                self.translate += 1.0;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class={css!(
                position: fixed;
                top: 0;
                left: 0;
                bottom: 0;
                right: 0;
                overflow: auto;
                overflow-x: hidden;
                background-color: #121212;
                display: flex;
                justify-content: center;
                align-items: center;
            )}>
                <div class={css!(
                    margin: auto;
                    text-align: center;
                    transition: 1s ease-in-out;
                    transform: translateX(${-self.translate * 100.0}%);
                    font-size: 100px;
                    user-select:none;
                    cursor: pointer;
                    z-index: 1;
                )}
                    onclick=self.link.callback(|_| Msg::TractorClicked)
                >
                    { "🚜" }
                </div>
                <div class={css!(
                    margin: auto;
                    transition: 1s ease-in-out;
                    transform: translateX(${-self.translate * 100.0}%);
                    font-size: 80px;
                    font-family: "Iosevka";
                    background: -webkit-linear-gradient(left, #880088, #4444aa, #008888, #00bb88);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    user-select: none;
                )}
                >
                    { "under construction" }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
