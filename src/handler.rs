use hyper;
use axum::{
    extract::{Json, State, RawBody}, 
    http::{header::HeaderMap, StatusCode},
};
use serde_json;
use serde_json::{Value, json};
use crate::enums::{InteractionType, InteractionCallbackType};
use crate::interaction::Interaction;
use crate::app::AppState;
use crate::utils::verify_signature;


pub (crate) async fn handler(
    headers: HeaderMap,
    State(state): State<AppState>, 
    RawBody(body): RawBody,
) -> (StatusCode, Json<Value>){
    let data = hyper::body::to_bytes(body).await.unwrap();

    let public_key_hex = state.public_key.as_str();
    let signature_hex = headers.get("X-Signature-Ed25519").unwrap().to_str().unwrap();
    let timestamp = headers.get("X-Signature-Timestamp").unwrap().as_bytes();

    let result = verify_signature(signature_hex, public_key_hex, &data, timestamp);
   
    if !result{
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "BadSignature"})));
    }

    let interaction = serde_json::from_slice::<Interaction>(&data).unwrap();

    match InteractionType::try_from(interaction.kind).unwrap()  {
        InteractionType::Ping => {
            println!("{}", json!({"type": InteractionCallbackType::Pong }));
            return (StatusCode::OK, Json(json!({"type": InteractionCallbackType::Pong as u8 })));
        },
        InteractionType::ApplicationCommand => {
            return (
                StatusCode::OK, 
                Json(json!({
                    "type": 4,
                    "data": {
                        "content": "Hello, World!"
                    }
                }))
            );
        },
        InteractionType::MessageComponent => todo!(),
        InteractionType::ApplicationCommandAutocomplete => todo!(),
        InteractionType::ModalSubmit => todo!(),
    }
    
}