use authorization::Authorization;
// use base64::{Engine as _, engine::general_purpose};
// use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

mod authorization;

// /// This is the main body for the function.
// /// Write your code inside it.
// /// There are some code example in the following URLs:
// /// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
// async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
fn main() {
    let auth_header = "Basic dGVzdGNsaWVudDp0ZXN0WHd1R1lpS1FUek9vbVJSc3JuR0psVXp2R09WVw==";

    // Step 1: Parse the client credentials from the given "authorization" property:
    let authorization = Authorization::from_basic_header(auth_header);
    // The Authorization header does not always have to be present!
    println!("Authorization: {:?}", authorization.unwrap());

    // Step 2: Load the OAuth client from the given "client_id" property:
    // Step 3: Validate the client ID, client secret, grant type and scopes:
    // Step 4: Read or Create access token:
    // Step 5: Return response:

//     // Extract some useful information from the request
//     let authorization = event
//         .query_string_parameters_ref()
//         .and_then(|params| params.first("authorization"))
//         .and_then(|auth| general_purpose::STANDARD.decode(auth))
//         .unwrap_or("world");
//     let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

//     // Return something that implements IntoResponse.
//     // It will be serialized to the right response event automatically by the runtime
//     let resp = Response::builder()
//         .status(200)
//         .header("content-type", "text/html")
//         .body(message.into())
//         .map_err(Box::new)?;
//     Ok(resp)
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::INFO)
//         // disable printing the name of the module in every log line.
//         .with_target(false)
//         // disabling time is handy because CloudWatch will add the ingestion time.
//         .without_time()
//         .init();

//     run(service_fn(function_handler)).await
// }
