use axum::async_trait;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

// Different from the video. Probably due to updates
// we do not need the generic <B>
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("--> {:<12} - mw_require_auth {ctx:?}", "MIDDLEWARE");

    //let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    //// Parse token
    //let (user_id, exp, sign) = auth_token
    //    .ok_or(Error::AuthFailNoAuthTokenCookie)
    //    .and_then(parse_token)?;

    // TODO we want to access this user_id downstream in the methods
    // so how do we do that? With extractors.
    // In this case we'll create a custom one

    ctx?; // Does the automatic validation and we'll be able to access ctx in methods

    Ok(next.run(req).await)
}

// Convert to middleware to cache the computation and avoid executing it
// multiple times
// --> EXTRACTOR    - ctx
// --> MIDDLEWARE   - mw_require_auth Ok(Ctx { user_id: 1 })
// --> EXTRACTOR    - ctx
// --> HANDLER      - create_ticket
// --> RES_MAPPER   - main_response_mapper
pub async fn mw_ctx_resolver(
    _mc: State<ModelController>, // We will eventually want to access DB from here
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    println!("--> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    // ??? Why do we duplicate this code??)
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO token components validation
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove cookie if something when wrong other than NoAuthCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store ctx_result in the request extension
    req.extensions_mut().insert(result_ctx); // like a dict by type

    Ok(next.run(req).await)
}

// Ctx extractor
// requires an async trait that is not supported by default
//
// https://docs.rs/axum/latest/axum/extract/index.html
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:<12} - ctx", "EXTRACTOR");
        // Previously we had token validation here but it was executed
        // multiple times
        //
        // Now we limit this to recover the results from the extensions
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequuestExt)?
            .clone()
    }
}

/// Parse token of format `user-[user-id].[expiration].[signature]`
///
/// TODO Should be changed for real token library
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
