use crate::model::conversation::{Conversation, Message};
use leptos::*;
use leptos_meta::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, _set_conversation) = create_signal(Conversation::new());
    let send = create_action(move | new_message: &String| {
        let user_message = Message{
            text: new_message.clone(),
            user: true,
        };
        _set_conversation.update(move |c|){
            c.message.push(user_message);
        }
        //TODO converse

    });
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet

        <Stylesheet id="leptos" href="/pkg/rusty-llama.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
        // {conversation.get()}
        // <ChatArea conversation/>
        // <TypeArea send/>
    }
}
