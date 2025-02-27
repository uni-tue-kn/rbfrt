use rbfrt::util::port_manager::{Port, Speed, FEC};
use rbfrt::util::PortManager;
use rbfrt::SwitchConnection;

const CONFIG_FILE: &str = "example.conf";

#[tokio::test]
async fn test_port_manager_init() -> Result<(), Box<dyn std::error::Error>> {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    PortManager::new(&switch).await;

    Ok(())
}

#[tokio::test]
async fn test_port_add_100g() -> Result<(), Box<dyn std::error::Error>> {
    let switch = SwitchConnection::builder("localhost", 50052)
        .device_id(0)
        .client_id(1)
        .config(CONFIG_FILE)
        .connect()
        .await?;

    let pm = PortManager::new(&switch).await;

    let port = Port::new(1, 0)
        .speed(Speed::BF_SPEED_100G)
        .fec(FEC::BF_FEC_TYP_NONE);

    pm.add_port(&switch, &port).await?;

    let port_list = pm.get_ports(&switch).await?;

    assert_eq!(port_list.len(), 1);

    let port_data = port_list.first().unwrap();

    assert_eq!(*port_data.get_speed(), Speed::BF_SPEED_100G);
    assert_eq!(*port_data.get_fec(), FEC::BF_FEC_TYP_NONE);

    pm.delete_port(&switch, &port).await?;

    let port_list = pm.get_ports(&switch).await?;

    assert_eq!(port_list.len(), 0);

    Ok(())
}
