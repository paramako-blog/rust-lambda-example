use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    event
        .payload
        .records
        .into_iter()
        .filter_map(|m| m.body)
        .for_each(|m| println!("{m}"));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env_var = std::env::var("MODE").expect("Environment error");
    println!("{env_var}");

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_function_handler() {
        let data = include_bytes!("fixtures/example-sqs-event.json");
        let parsed: SqsEvent = serde_json::from_slice(data).unwrap();
        let context = lambda_runtime::Context::default();
        let event = LambdaEvent::new(parsed, context);

        function_handler(event).await.expect("failed to handle event");
    }
}
