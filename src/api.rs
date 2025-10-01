use leptos::prelude::*;

#[server(Converse, "/api")]
pub async fn converse(prompt: String) -> Result<String, ServerFnError> {
    use actix_web::web::Data;
    use kalosm::language::*;
    use leptos_actix::extract;
    use std::sync::Mutex;

    let chat: Data<Mutex<Chat<Llama>>> = extract().await?;
    let mut chat_guard = chat.lock().unwrap();

    let usr_chat_msg = ChatMessage::new(MessageType::UserMessage, prompt.clone());
    let res = chat_guard.add_message(usr_chat_msg).all_text().await;

    Ok(res)
}
