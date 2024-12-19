mod ping;

use std::str::FromStr;
use sysinfo::{Components, Disks, Networks, System};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 系统信息
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("总共内存: {}GB", sys.total_memory() as f64 / 1073741824.0);
    println!("内存使用 : {}GB", sys.used_memory() as f64 / 1073741824.0);
    println!("使用率: {}%", (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0);
    println!("总共交换内存:{}GB", sys.total_swap() as f64 / 1073741824.0);
    println!("使用交换内存:{}GB", sys.used_swap() as f64 / 1073741824.0);

    // Display system information:
    println!("系统:{:?}", System::name().unwrap());
    println!("架构:{:?}", System::cpu_arch());
    println!("内核:{:?}", System::kernel_version().unwrap());
    println!("版本:{:?}", System::os_version().unwrap());
    println!("主机名:{:?}", System::host_name().unwrap());

    // Number of CPUs:
    println!("CPU核数: {}", sys.cpus().len());
    println!("CPU使用率: {}%", sys.global_cpu_usage());
    for cpu in sys.cpus() {
        println!("CPU使用率: {}% 名字:{} 品牌:{} 供应商:{} 频率:{}Mhz", cpu.cpu_usage(),cpu.name(), cpu.brand(), cpu.vendor_id(), cpu.frequency());
    }

    // We display all disks' information:
    println!("=> 磁盘情况:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "全部空间:{}GB 有效空间:{}GB 占用率:{}%",
            disk.total_space() as f64 / 1073741824.0,
            disk.available_space() as f64 / 1073741824.0,
            (disk.available_space() as f64 / disk.total_space() as f64) * 100.0
        );
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> 网络情况:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} GB (下载) / {} GB (上传)",
            (data.total_received() * 8) / 1000000,
            (data.total_transmitted() * 8) / 1000000,
        );
    }

    Ok(())
}
