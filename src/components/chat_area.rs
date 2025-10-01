use leptos::{html::Div, prelude::*};

use crate::model::conversation::Conversation;

const USER_MSG_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MSG_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 bg-white" node_ref=chat_div_ref>
            {move || conversation.get().messages.iter().map(move |msg| {
                let class_str = if msg.user { USER_MSG_CLASS} else {MODEL_MSG_CLASS};
                view! {
                    <div class={class_str}>
                        {msg.text.clone()}
                    </div>
                }
            }).collect::<Vec<_>>()

            }
        </div>
    }
}
