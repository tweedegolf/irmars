use std::time::Duration;

use irma::{AttributeRequest, DisclosureRequestBuilder, IrmaClient};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Create an irma client
    let client = IrmaClient::new("http://localhost:8088/").unwrap();

    // Setup our request
    let request = DisclosureRequestBuilder::new()
        .add_discon(vec![vec![AttributeRequest::Simple(
            "pbdf.sidn-pbdf.email.email".into(),
        )]])
        .build();

    // Start the session
    let session = client
        .request(&request)
        .await
        .expect("Failed to start session");

    // Encode the session pointer
    let sessionptr = serde_json::to_string(&session.session_ptr).unwrap();

    // Render a qr
    let qr = qrcode::QrCode::new(sessionptr)
        .unwrap()
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("\n\n{}", qr);

    // Periodically poll if the session was succesfully concluded
    let result = loop {
        match client.result(&session.token).await {
            Ok(result) => break result,
            Err(irma::Error::SessionNotFinished(_)) => {}
            Err(v) => panic!("{}", v),
        }

        sleep(Duration::from_secs(2)).await;
    };

    println!("Session result: {:?}", result);
}
