use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, ctx: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        bail!("Empty first name");
    }
    Ok(CustomOutput{
        message: format!("Hello, {}!", e.first_name),
    })
}