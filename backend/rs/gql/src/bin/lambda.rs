use lambda_runtime::{error::HandlerError, lambda, Context};
use log::error;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(
    e: CustomEvent,
    c: Context,
) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        return Err("Empty first name".into());
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}
