use crate::eventbriteapiv3public::{Attendee, Event, Pagination};
use failure::Error;
use serde::{Deserialize, Serialize};
use std::env;

pub mod eventbriteapiv3public;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersMeEvents {
    pagination: Pagination,
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeesPage {
    pub pagination: Pagination,
    pub attendees: Vec<Attendee>,
}

impl AttendeesPage {
    pub fn url(events: i64) -> String {
        format!("/events/{}/attendees", events)
    }
}

pub struct Eventbrite {
    token: String,
}

impl Eventbrite {
    fn base_url(&self) -> String {
        "https://www.eventbriteapi.com/v3".to_string()
    }

    /// use the env EVENTBRITE_API_KEY as api key
    pub fn new_from_env() -> Result<Eventbrite, Error> {
        dotenv::dotenv()?;
        Ok(Self::new(env::var("EVENTBRITE_API_KEY")?))
    }

    pub fn new(token: String) -> Eventbrite {
        Eventbrite { token }
    }

    pub fn events(&self) -> Result<UsersMeEvents, Error> {
        reqwest::get(&format!(
            "{}/users/me/events/?token={}",
            self.base_url(),
            self.token
        ))?
        .json()
        .map_err(|e| failure::format_err!("test {:?}", e))
    }

    pub fn attendees(
        &self,
        events: i64,
        continuation: Option<String>,
    ) -> Result<AttendeesPage, Error> {
        let continuation = if let Some(continuation) = continuation {
            format!("continuation={}&", continuation)
        } else {
            "".to_string()
        };

        reqwest::get(&format!(
            "{}/events/{}/attendees/?{}token={}",
            self.base_url(),
            events,
            continuation,
            self.token
        ))?
        .json()
        .map_err(|e| failure::format_err!("test {:?}", e))
    }
}
