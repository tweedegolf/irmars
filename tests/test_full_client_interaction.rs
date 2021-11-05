use std::{fs, process::Stdio, time::Duration};

use irma::{
    AttributeRequest, CredentialBuilder, DisclosureRequestBuilder, IrmaClient,
    IssuanceRequestBuilder, SessionStatus, SignatureRequestBuilder,
};
use serial_test::serial;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
    time::sleep,
};

#[test]
#[serial]
fn test_full_client_interaction() {
    if option_env!("RUN_INTEGRATION_TESTS").is_some() {
        tokio_test::block_on(async {
            //
            // Setup
            //
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

            // Reset client emulator
            fs::remove_dir_all("temp_testing/client").unwrap();
            fs::create_dir("temp_testing/client").unwrap();

            // Create an irma client
            let client = IrmaClient::new("http://localhost:8088/").unwrap();

            //
            // Issuance
            //

            // Setup our request
            let request = IssuanceRequestBuilder::new()
                .add_credential(
                    CredentialBuilder::new("irma-demo.sidn-pbdf.email".into())
                        .attribute("email".into(), "test@example.com".into())
                        .build(),
                )
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

            let mut client_emulator = Command::new("./test_tools/client_emulator/client_emulator")
                .stdin(Stdio::piped())
                .spawn()
                .expect("Could not start client emulator");

            let mut client_emulator_input = client_emulator.stdin.take().unwrap();

            let session_ptr = serde_json::to_string(&session.session_ptr).unwrap();
            client_emulator_input
                .write_all(format!("{}\n", session_ptr).as_bytes())
                .await
                .unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Connected);

            client_emulator_input
                .write_all("proceed\n".as_bytes())
                .await
                .unwrap();
            client_emulator.wait().await.unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Done);

            //
            // Disclosure
            //

            let request = DisclosureRequestBuilder::new()
                .add_discon(vec![vec![AttributeRequest::Simple(
                    "irma-demo.sidn-pbdf.email.email".into(),
                )]])
                .build();

            let session = client
                .request(&request)
                .await
                .expect("Failed to start session");

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Initialized);

            let mut client_emulator = Command::new("./test_tools/client_emulator/client_emulator")
                .stdin(Stdio::piped())
                .spawn()
                .expect("Could not start client emulator");

            let mut client_emulator_input = client_emulator.stdin.take().unwrap();

            let session_ptr = serde_json::to_string(&session.session_ptr).unwrap();
            client_emulator_input
                .write_all(format!("{}\n", session_ptr).as_bytes())
                .await
                .unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Connected);

            client_emulator_input
                .write_all("proceed\n".as_bytes())
                .await
                .unwrap();
            client_emulator.wait().await.unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Done);

            client
                .result(&session.token)
                .await
                .expect("Could not fetch result");

            client
                .result(&session.token)
                .await
                .expect("Could not fetch result");

            //
            // Signature
            //

            let request = SignatureRequestBuilder::new("testmessage".into())
                .add_discon(vec![vec![AttributeRequest::Simple(
                    "irma-demo.sidn-pbdf.email.email".into(),
                )]])
                .build();

            let session = client
                .request(&request)
                .await
                .expect("Failed to start session");

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Initialized);

            let mut client_emulator = Command::new("./test_tools/client_emulator/client_emulator")
                .stdin(Stdio::piped())
                .spawn()
                .expect("Could not start client emulator");

            let mut client_emulator_input = client_emulator.stdin.take().unwrap();

            let session_ptr = serde_json::to_string(&session.session_ptr).unwrap();
            client_emulator_input
                .write_all(format!("{}\n", session_ptr).as_bytes())
                .await
                .unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Connected);

            client_emulator_input
                .write_all("proceed\n".as_bytes())
                .await
                .unwrap();
            client_emulator.wait().await.unwrap();

            sleep(Duration::from_secs(1)).await;

            let status = client
                .status(&session.token)
                .await
                .expect("Could not fetch status");
            assert_eq!(status, SessionStatus::Done);

            client
                .result(&session.token)
                .await
                .expect("Could not fetch result");

            //
            // Teardown
            //
            irmaserver.kill().await.expect("Error killing irma server");
        });
    }
}
