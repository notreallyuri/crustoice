use crate::state::{SharedState, Tx};
use axum::extract::ws::Message;
use shared::{protocol::ServerMessage, structures::UserId};

pub async fn broadcast_global(
    state: &SharedState,
    msg: ServerMessage,
    exclude_user: Option<&UserId>,
) {
    let json = serde_json::to_string(&msg).unwrap();

    let receivers: Vec<Tx> = {
        let state_guard = state.lock().await;
        state_guard
            .sessions
            .iter()
            .filter_map(|(user_id, session)| {
                if exclude_user == Some(user_id) {
                    None
                } else {
                    Some(session.tx.clone())
                }
            })
            .collect()
    };

    for tx in receivers {
        let _ = tx.send(Message::Text(json.clone().into()));
    }
}
