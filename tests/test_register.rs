use rbfrt::table::ToBytes;
use rbfrt::{register, SwitchConnection};

const CONFIG_FILE: &str = "traffic_gen.conf";

#[tokio::test]
async fn test_write_entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    let req = register::Request::new("ingress.p4tg.rx_seq")
        .index(5)
        .data("ingress.p4tg.rx_seq.f1", 20);

    switch.write_register_entry(req.clone()).await?;

    let register = switch.get_register_entry(req).await?;

    // we queried a single register entry
    assert_eq!(register.entries().len(), 1);

    let reg_entry = register.get(5).unwrap();

    assert_eq!(
        reg_entry
            .get("ingress.p4tg.rx_seq.f1")
            .unwrap()
            .get(0)
            .unwrap()
            .to_u32(),
        20
    );

    Ok(())
}
