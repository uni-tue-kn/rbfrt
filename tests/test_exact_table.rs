use rbfrt::table::{ActionData, MatchValue, ToBytes};
use rbfrt::{table, SwitchConnection};

const CONFIG_FILE: &str = "example.conf";

#[tokio::test]
async fn test_write_entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    let req = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req).await?;

    let read_req = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10));

    let entries = switch.get_table_entry(read_req).await?;

    assert_eq!(entries.len(), 1);

    let entry = entries.get(0).unwrap();

    let key = entry
        .get_key("ig_intr_md.ingress_port")?
        .get_exact_value()
        .to_u32();
    let action_value = entry.get_action_data("e_port")?.as_u32();

    assert_eq!(key, 10);
    assert_eq!(action_value, 20);

    Ok(())
}

#[tokio::test]
async fn test_update_entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entry(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    let val = entries.get(0).unwrap().get_action_data("e_port")?.as_u32();

    assert_eq!(val, 20);

    // update entry
    let update = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
        .action("ingress.do_forward")
        .action_data("e_port", 25);

    switch.update_table_entry(update).await?;

    // verify entry
    let entries = switch.get_table_entry(req).await?;

    let val = entries.get(0).unwrap().get_action_data("e_port")?.as_u32();

    assert_eq!(val, 25);

    Ok(())
}

#[tokio::test]
async fn test_delete_entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entry(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    // delete entry
    let delete = table::Request::new("ingress.exact_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
        .action("ingress.do_forward")
        .action_data("e_port", 25);

    switch.delete_table_entry(delete).await?;

    // verify
    let entries = switch.get_table_entry(req).await?;

    assert_eq!(entries.len(), 0);

    Ok(())
}
