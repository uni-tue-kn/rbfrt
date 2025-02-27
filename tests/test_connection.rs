use rbfrt::SwitchConnection;

const CONFIG_FILE: &str = "example.conf";

#[tokio::test]
async fn test_connection() {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await;

    assert!(switch.is_ok());
}

#[tokio::test]
async fn test_connection_failure() {
    let switch = SwitchConnection::builder("localhost", 50054)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await;

    assert!(switch.is_err());
}
