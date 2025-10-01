use leptos::{html::Input, prelude::*};

#[component]
pub fn PromptArea(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_field_ref = NodeRef::<Input>::new();

    view! {
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t">
            <form class="flex flex-wrap justify-center items-center space-x-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_field_ref.get().expect("input field to exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input type="text" class="w-2/3 p-4 border rounded-full input-field" placeholder="Enter ye quest..." node_ref=input_field_ref />
                <button type="submit" class="text-white rounded-full cursor-pointer bg-blue-700 rounded-full px-5 py-2.5 text-center mb-2">Send</button>
            </form>
        </div>
    }
}
