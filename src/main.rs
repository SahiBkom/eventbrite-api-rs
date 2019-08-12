use crate::eventbriteapiv3public::Event;
use crate::eventbriteapiv3public::Pagination;
use crate::eventbriteapiv3public::Attendee;
use failure::Error;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
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
    attendees: Vec<Attendee>
}

fn main() -> Result<(), Error> {
    dotenv().ok();
    println!("EVENTBRITE_API_KEY: {}", env::var("EVENTBRITE_API_KEY").unwrap());
    
    let eb = Eventbrite::new(env::var("EVENTBRITE_API_KEY").unwrap());

    println!("{:?}", eb.attendees(60371513823));

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
        .json().map_err(|e| failure::format_err!("test {:?}", e))
    }

    fn attendees(&self, events: i64) -> Result<AttendeesPage, Error> {
        reqwest::get(&format!(
            "{}/events/{}/attendees/?token={}",
            self.base_url(),
            events,
            self.token
        ))?
        .json().map_err(|e| failure::format_err!("test {:?}", e))
    }
    
}
