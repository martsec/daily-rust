use axum::{
    extract::FromRef,
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    ctx::Ctx,
    model::{ModelController, Ticket, TicketForCreate},
    Result,
};

// If we have more than one state, we can do the following
// FromRef is an axum macro (need to add the feature) `macros` which makes
// all properties a sub state that gets injected automatically as needed
//
#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
    // ...
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        // It will magically obtain the correct state in each of the routes
        .with_state(app_state)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("--> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
