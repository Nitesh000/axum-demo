use axum::{extract::{State, Path}, Json, Router, routing::{post, delete}};

use crate::{model::{ModelController, Ticket, TicketForCreate}, Result};

async fn create_ticket( State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("-->> {:<12} - list_ticket", "HANDLER");
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc) : State<ModelController>,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - list_ticket", "HANDLER");
    
    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}

// #[derive(Clone, FormRef)] // to use this add feature "macro" to the axum
// struct AppState {
//     mc: ModelController,
// }

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc};
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
        //.with_state(app_state)
}
