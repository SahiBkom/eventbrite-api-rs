use crate::eventbriteapiv3public::Attendee;
use crate::eventbriteapiv3public::Event;
use crate::eventbriteapiv3public::Pagination;
use dotenv::dotenv;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::env;

pub mod eventbriteapiv3public;

#[derive(Debug, Serialize, Deserialize)]
struct UsersMeEvents {
    pagination: Pagination,
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendeesPage {
    pagination: Pagination,
    attendees: Vec<Attendee>,
}

impl AttendeesPage {
    pub fn url(events: i64) -> String {
        format!("/events/{}/attendees", events)
    }
}

fn main() -> Result<(), Error> {
    dotenv().ok();
    println!(
        "EVENTBRITE_API_KEY: {}",
        env::var("EVENTBRITE_API_KEY").unwrap()
    );

    let eb = Eventbrite::new(env::var("EVENTBRITE_API_KEY").unwrap());

    // println!("{:?}", eb.attendees(60371513823));

    let mut attendees = eb.attendees(60371513823, None)?;
    println!("=> {:?}", attendees.attendees[5]);
    // println!("=> {:?}", attendees);
    // while let Some(continuation) = attendees.pagination.continuation {
    //     attendees = eb.attendees(60371513823, Some(continuation))?;
    //     println!("=> {:?}", attendees);
    // }

    Ok(())
}

struct Eventbrite {
    token: String,
}

impl Eventbrite {
    fn base_url(&self) -> String {
        "https://www.eventbriteapi.com/v3".to_string()
    }

    fn new(token: String) -> Eventbrite {
        Eventbrite { token }
    }

    fn events(&self) -> Result<UsersMeEvents, Error> {
        reqwest::get(&format!(
            "{}/users/me/events/?token={}",
            self.base_url(),
            self.token
        ))?
        .json()
        .map_err(|e| failure::format_err!("test {:?}", e))
    }

    fn attendees(&self, events: i64, continuation: Option<String>) -> Result<AttendeesPage, Error> {
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
