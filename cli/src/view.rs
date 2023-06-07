use ufmclient::{UFMError, UFMConfig};

pub async fn run(conf: UFMConfig, pkey: &String) -> Result<(), UFMError> {
    let ufm = ufmclient::connect(conf)?;
    let p = ufm.get_partition(pkey).await?;
    let ps = ufm.list_port(p.pkey).await?;

    println!("{:15}: {}", "Name", p.name);
    println!("{:15}: {}", "Pkey", p.pkey.to_string());
    println!("{:15}: {}", "IPoIB", p.ipoib);
    println!("{:15}: {}", "MTU", p.qos.mtu_limit);
    println!("{:15}: {}", "Rate Limit", p.qos.rate_limit);
    println!("{:15}: {}", "Service Level", p.qos.service_level);
    println!("{:15}: ", "Ports");

    println!(
        "    {:<20}{:<20}{:<10}{:<20}{:<10}{:<15}{:<10}{:<20}",
        "GUID", "ParentGUID", "PortType", "SystemID", "LID", "SystemName", "LogState", "Name", 
    );
    for port in ps {
        println!("{}", port.to_string());
    }

    Ok(())
}
