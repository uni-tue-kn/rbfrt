use rbfrt::{SwitchConnection, table};
use rbfrt::table::{ActionData, MatchValue, ToBytes};

const CONFIG_FILE: &str = "example.conf";

#[tokio::test]
async fn test_write_entry() -> Result<(), Box<dyn std::error::Error>>{
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    let req = table::Request::new("ingress.lpm_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::lpm(5, 2))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req).await?;

    let read_req = table::Request::new("ingress.lpm_forward");

    let entries = switch.get_table_entry(read_req).await?;

    // default action exists always
    assert_eq!(entries.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_update_entry() -> Result<(), Box<dyn std::error::Error>>{
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.lpm_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::lpm(5, 2))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entry(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    let val = entries.get(0).unwrap().get_action_data("e_port")?.get_data().to_u32();

    assert_eq!(val, 20);

    // update entry
    let update = table::Request::new("ingress.lpm_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::lpm(5, 2))
        .action("ingress.drop");

    switch.update_table_entry(update).await?;

    // verify entry
    let entries = switch.get_table_entry(req).await?;

    let val = entries.get(0).unwrap().get_action_name();

    assert_eq!(val, "ingress.drop");

    Ok(())
}

#[tokio::test]
async fn test_delete_entry() -> Result<(), Box<dyn std::error::Error>>{
    let mut switch = SwitchConnection::new("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    // insert entry
    let req = table::Request::new("ingress.lpm_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::lpm(5, 2))
        .action("ingress.do_forward")
        .action_data("e_port", 20);

    switch.write_table_entry(req.clone()).await?;

    // verify entry
    let entries = switch.get_table_entry(req.clone()).await?;

    assert_eq!(entries.len(), 1);

    // delete entry
    let delete = table::Request::new("ingress.lpm_forward")
        .match_key("ig_intr_md.ingress_port", MatchValue::lpm(5, 2));

    switch.delete_table_entry(delete).await?;

    // verify
    let entries = switch.get_table_entry(req).await?;

    assert_eq!(entries.len(), 0);

    Ok(())
}



