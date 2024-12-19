# rs-point

## ping example
```rust
mod ping;

use std::net::IpAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ping 服务
    let target_ip: IpAddr = "8.8.8.8".parse().unwrap();
    match ping::ping(target_ip).await? {
        Some(duration) => println!("Ping successful, RTT: {:?}", duration),
        None => println!("Ping failed or timed out"),
    }
    Ok(())
}


```