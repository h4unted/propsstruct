use yew::prelude::*;

mod components;
use crate::components::atoms::main_title::MainTitle;
use crate::components::molecules::custom_form::CustomForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <div>
                <MainTitle /> 
            </div>
            <div>
                <CustomForm />
            </div>
        </>
    }
}