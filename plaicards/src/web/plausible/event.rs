use gloo_net::http::Request;
use gloo_utils::format::JsValueSerdeExt;
use leptos::{self, document, window};
use serde::{Deserialize, Serialize};
/// Track pageviews and custom events to plausible
///
///
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Navigator;

/// Main intro class handling Plausible events API.
///
/// ```rust
/// use plaicards::web::plausible::event::Plausible;
///
/// let p = Plausible::new("your_domain");
///
/// p.pageview.send.await;
/// ```
pub struct Plausible {
    /// This domain name you used when you added your site to your Plausible account
    domain: String,
    /// Plausibe url for custom instances. By `new()` constructor sets it as `https://plausible.io`
    plausible_url: String,
}

impl Plausible {
    // Handy methods
    //
    #[must_use]
    pub fn link_click(&self, outbound_url: String) -> EventBuilder {
        let mut builder = self.build_event(EventName::OutboundLinkClick);
        builder.props(HashMap::from([(String::from("url"), outbound_url.into())]));
        builder
    }

    #[must_use]
    pub fn pageview(&self) -> EventBuilder {
        self.build_event(EventName::Pageview)
    }

    #[must_use]
    pub fn event(&self, name: String) -> EventBuilder {
        self.build_event(EventName::Custom(name))
    }
}

impl Plausible {
    #[must_use]
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            plausible_url: "https://plausible.io".into(),
        }
    }

    #[must_use]
    pub fn new_private(domain: &str, instance_url: &str) -> Self {
        Self {
            domain: domain.into(),
            plausible_url: instance_url.into(),
        }
    }

    fn default_url() -> String {
        window()
            .location()
            .href()
            .expect("ERROR with plausible event: url")
    }

    fn build_event(&self, name: EventName) -> EventBuilder {
        let header = PlausibleHeader {
            user_agent: window()
                .navigator()
                .user_agent()
                .unwrap_or_else(|_| "ERROR".into()),
            // FIXME this is wrong
            x_forwarded_for: window()
                .location()
                .href()
                .expect("Error with plausible event"),
        };
        let referrer = document().referrer();
        let body = PlausiblePayload {
            name: name.into(),
            url: Self::default_url(),
            domain: self.domain.clone(),
            referrer: if referrer.is_empty() {
                None
            } else {
                Some(referrer)
            },
            props: None,
            revenue: None,
            screen_width: None,
        };

        EventBuilder {
            header,
            body,
            plausible_url: self.plausible_url.clone(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
struct PlausibleHeader {
    pub user_agent: String,
    pub x_forwarded_for: String,
}

impl PlausibleHeader {
    #[must_use]
    pub const fn new(user_agent: String, x_forwarded_for: String) -> Self {
        Self {
            user_agent,
            x_forwarded_for,
        }
    }
}

/// Revenue data for this event.
/// This can be attached to goals and custom events to track revenue attribution.
///
/// In the case of an invalid currency or amount, the event is still recorded and
/// the API returns HTTP 202, but revenue data associated with it is discarded.
///
/// Not available to the community edition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueValue {
    currency: String,
    amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlausiblePayload {
    /// Name of the event
    pub name: String,
    /// Domain name of the site in plausible
    pub domain: String,
    /// URL of the page where the event was triggered.
    /// By default it will be javascript's `window.location.href`
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<usize>,
    /// Custom properties only accepts scalar values such as strings, numbers and booleans.
    /// Data structures such as objects, arrays etc. aren't accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<HashMap<String, PropValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue: Option<RevenueValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum EventName {
    // TODO serialize them to string
    Pageview,
    // TODO need to add property `url`
    OutboundLinkClick,
    FileDownload,
    Purchase,
    NotFound,
    Custom(String),
}

#[allow(clippy::from_over_into)]
impl Into<String> for EventName {
    fn into(self) -> String {
        match self {
            Self::Pageview => "pageview".to_string(),
            Self::OutboundLinkClick => "Outbound Link: Click".to_string(),
            Self::FileDownload => "File Download".to_string(),
            Self::Purchase => "Purchase".to_string(),
            Self::NotFound => "404".to_string(),
            Self::Custom(s) => s,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct EventBuilder {
    plausible_url: String,
    header: PlausibleHeader,
    body: PlausiblePayload,
}

impl EventBuilder {
    pub fn props(&mut self, props: HashMap<String, PropValue>) -> &mut Self {
        // TODO avoid overwritting and support adding if body.props already exists
        match &mut self.body.props {
            None => self.body.props = Some(props),
            Some(existing) => {
                for (k, v) in props.clone().drain() {
                    existing.insert(k, v);
                }
            }
        };
        self
    }

    pub fn revenue(&mut self, revenue: RevenueValue) -> &mut Self {
        self.body.revenue = Some(revenue);
        self
    }

    pub fn referrer(&mut self, referrer: String) -> &mut Self {
        self.body.referrer = Some(referrer);
        self
    }

    pub fn screen_width(&mut self, screen_width: usize) -> &mut Self {
        self.body.screen_width = Some(screen_width);
        self
    }

    /// Use it to specify custom locations for your page URL.
    ///
    /// For example if they include identifiers lile PII and UUID and you don't want to send those.
    /// You can send just `/user` to avoid sending sensitive data and improve
    /// Top Pages statistics.
    pub fn url(&mut self, url: &str) -> &mut Self {
        let url_with_params = format!(
            "{url}{}",
            window().location().search().expect("ERR with plausible")
        );
        self.body.url = url_with_params;
        self
    }

    pub async fn send(self) {
        // TODO disable sending event if localhost like done in the official script
        // TODO don't send pageview event if already visited before (window.history)
        // FIXME this from_serde should work but returns JSValue(Object(...)) which fails
        let body = JsValue::from_serde(&self.body).expect("ERR serializing");
        let body = JsValue::from_str(&serde_json::to_string(&self.body).expect("ERR"));

        let resp = Request::post(&format!("{}/api/event", self.plausible_url))
            .referrer_policy(web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin)
            .mode(web_sys::RequestMode::NoCors)
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "application/json")
            .body(body);

        let resp = resp.expect("Error building the body").send().await;
    }
}

/// Custom properties only accepts scalar values such as strings, numbers and booleans.
/// Data structures such as objects, arrays etc. aren't accepted.
// Implementation on how to constrain types easily from: https://stackoverflow.com/a/52582432/11767294
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropValue {
    // string
    String(String),

    // bool
    Bool(bool),

    // numbers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),

    F32(f32),
    F64(f64),
}

impl From<String> for PropValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<bool> for PropValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<u8> for PropValue {
    fn from(u: u8) -> Self {
        Self::U8(u)
    }
}

impl From<u16> for PropValue {
    fn from(u: u16) -> Self {
        Self::U16(u)
    }
}

impl From<u32> for PropValue {
    fn from(u: u32) -> Self {
        Self::U32(u)
    }
}

impl From<u64> for PropValue {
    fn from(u: u64) -> Self {
        Self::U64(u)
    }
}

impl From<u128> for PropValue {
    fn from(u: u128) -> Self {
        Self::U128(u)
    }
}

impl From<usize> for PropValue {
    fn from(u: usize) -> Self {
        Self::Usize(u)
    }
}

impl From<i8> for PropValue {
    fn from(i: i8) -> Self {
        Self::I8(i)
    }
}

impl From<i16> for PropValue {
    fn from(i: i16) -> Self {
        Self::I16(i)
    }
}

impl From<i32> for PropValue {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<i64> for PropValue {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<i128> for PropValue {
    fn from(i: i128) -> Self {
        Self::I128(i)
    }
}

impl From<isize> for PropValue {
    fn from(i: isize) -> Self {
        Self::Isize(i)
    }
}

impl From<f32> for PropValue {
    fn from(f: f32) -> Self {
        Self::F32(f)
    }
}

impl From<f64> for PropValue {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}
