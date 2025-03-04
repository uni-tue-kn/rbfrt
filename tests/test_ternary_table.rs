use rbfrt::table::{MatchValue, ToBytes};
use rbfrt::{table, SwitchConnection};

const CONFIG_FILE: &str = "example.conf";

#[tokio::test]
async fn test_write_entry() -> Result<(), Box<dyn std::error::Error>> {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    let req = table::Request::new("ingress.ternary_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::ternary(7, 2))
        .match_key("$MATCH_PRIORITY", MatchValue::exact(1))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req).await?;

    let read_req = table::Request::new("ingress.lpm_forward");

    let entries = switch.get_table_entries(read_req).await?;

    assert_eq!(entries.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_update_entry() -> Result<(), Box<dyn std::error::Error>> {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.ternary_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::ternary(5, 2))
        .match_key("$MATCH_PRIORITY", MatchValue::exact(1))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entries(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    let val = entries
        .first()
        .unwrap()
        .get_action_data("e_port")?
        .get_data()
        .to_u32();

    assert_eq!(val, 20);

    // update entry
    let update = table::Request::new("ingress.ternary_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::ternary(5, 2))
        .match_key("$MATCH_PRIORITY", MatchValue::exact(1))
        .action("ingress.drop");

    switch.update_table_entry(update).await?;

    // verify entry
    let entries = switch.get_table_entries(req).await?;

    let val = entries.first().unwrap().get_action_name();

    assert_eq!(val, "ingress.drop");

    Ok(())
}

#[tokio::test]
async fn test_delete_entry() -> Result<(), Box<dyn std::error::Error>> {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.ternary_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::ternary(5, 2))
        .match_key("$MATCH_PRIORITY", MatchValue::exact(1))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entries(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    // delete entry
    let delete = table::Request::new("ingress.ternary_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::ternary(5, 2))
        .match_key("$MATCH_PRIORITY", MatchValue::exact(1));

    switch.delete_table_entry(delete).await?;

    // verify
    let entries = switch.get_table_entries(req).await?;

    assert_eq!(entries.len(), 0);

    Ok(())
}
