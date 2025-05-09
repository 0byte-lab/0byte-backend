use axum::response::Json as JsonResponse;

pub async fn hello() -> JsonResponse<String> {
    let message = "hello there, welcome to 0byte";
    JsonResponse(message.to_string())
}
