use std::process::Stdio;

use irma::{AttributeRequest, DisclosureRequestBuilder, IrmaClient, SessionStatus};
use serial_test::serial;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[test]
#[serial]
fn test_cancel() {
    if option_env!("RUN_INTEGRATION_TESTS").is_some() {
        tokio_test::block_on(async {
            let mut irmaserver = Command::new("irma")
                .arg("server")
                .stderr(Stdio::piped())
                .spawn()
                .expect("Could not start irma server");

            let irmaserver_stderr = irmaserver
                .stderr
                .take()
                .expect("No stderr available from irma server");
            let mut irmaserver_lines = BufReader::new(irmaserver_stderr).lines();
            loop {
                let line = irmaserver_lines
                    .next_line()
                    .await
                    .expect("Error reading from irma server stderr")
                    .expect("No line recieved");
                if line.contains("Server listening") {
                    break;
                }
            }

            println!("Server started");

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

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Initialized);

            client.cancel(&session.token).await.expect("Cancel failed");

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Cancelled);

            irmaserver.kill().await.expect("Error killing irma server");
        });
    }
}
