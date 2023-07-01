use yew::prelude::*;
use crate::components::atoms::off_key::KeyImage;



#[function_component(CustomForm)]
pub fn custom_form() -> Html {   
    let firststate = use_state( || "normal".to_owned()); 
    let cloned_firststate = firststate.clone(); 
        let opacity_change = Callback::from(move |kind| {
            cloned_firststate.set(kind);
        });                       

    html! {
        <form>
            <KeyImage opacity={ (*firststate).clone() } handle_onchange={opacity_change}/>
            <h1 > 
                {"This is a prologue"} 
            </h1>
        </form>
    }
}