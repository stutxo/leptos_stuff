use leptos::{
    component, create_node_ref, create_signal, ev::SubmitEvent, html::Input, spawn_local, view,
    IntoView, NodeRef,
};

use reqwest_wasm::Client;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h6>"metabase chart test"</h6>
        <MetabaseComponent/>
    }
}

#[component]
fn MetabaseComponent() -> impl IntoView {
    let (chart, set_name) = create_signal("".to_string());

    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().unwrap().value();

        if value.is_empty() {
            return;
        }

        spawn_local(async move {
            let client = Client::new();
            let res = client
                .post("https://httpbin.org/post")
                .body(value)
                .send()
                .await
                .unwrap();
            set_name(res.text().await.unwrap());
        });
    };

    let iframe = move || {
        if !chart().is_empty() {
            view! {
                <p>"Chart Name: " {chart}</p>
                <iframe src="https://bitcoinity.org/markets" width="100%" height="800"></iframe>
            }
        } else {
            view! {
                <p>"Chart Name: None Specified"</p>
                <iframe src="" width="100%" height="800"></iframe>
            }
        }
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=chart
                node_ref=input_element
            />

            <input type="submit" value="Submit"/>

        </form>

        {iframe}
    }
}
