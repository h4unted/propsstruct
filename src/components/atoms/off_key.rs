use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq,)]
pub struct Props {
    pub opacity: String,
    pub handle_onchange: Callback<String>,
    
}

#[styled_component(KeyImage)]
pub fn key_image(props: &Props) -> Html {    
    let stylesheet = style!(
        r#"
        .normal {
            opacity: 1;
        }

        .off {
            opacity: 0;
        }
    "#
    ) .unwrap();
    let handle_onchange = props.handle_onchange.clone();
    let onclick = Callback::from(move |_| {
        let value = String::from("off");
        handle_onchange.emit(value);
    }); 

    html! {
        <div class={stylesheet}>
            <img class={&props.opacity} src="pictures/offkey.png" {onclick} alt="offkey image" />
        </div>
    }
}
