use ::eventbrite_api::Eventbrite;
use failure::Error;

fn main() -> Result<(), Error> {
    let eb = Eventbrite::new_from_env()?;

    let mut attendees = eb.attendees(60371513823, None)?;
    println!("=> {:?}", attendees.attendees[5]);
    // println!("=> {:?}", attendees);
    // while let Some(continuation) = attendees.pagination.continuation {
    //     attendees = eb.attendees(60371513823, Some(continuation))?;
    //     println!("=> {:?}", attendees);
    // }

    Ok(())
}
