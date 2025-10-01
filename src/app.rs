use leptos::prelude::*;
use leptos_meta::*;

use crate::{
    api::converse,
    components::{chat_area::ChatArea, prompt_area::PromptArea},
    model::conversation::{Conversation, Message},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (conversation, set_conversation) = signal(Conversation::new());
    let send_conversation = Action::new(move |new_msg: &String| {
        let user_msg = Message {
            text: new_msg.clone(),
            user: true,
        };
        set_conversation.update(|c| {
            c.messages.push(user_msg);
        });

        converse(new_msg.clone())
    });

    Effect::new(move |_| {
        if let Some(_) = send_conversation.input().get() {
            let load_msg = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(load_msg);
            });
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(res)) = send_conversation.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = res;
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/crabby-prompt.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Title text="Crabby Prompt, now with some intelligence"/>
        <ChatArea conversation/>
        <PromptArea send=send_conversation/>
    }
}
