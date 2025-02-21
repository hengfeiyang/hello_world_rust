mod db;
mod sysinfo;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cpu_limit = sysinfo::get_cpu_limit();
    let memory_limit = sysinfo::get_memory_limit();
    let hostname = sysinfo::get_hostname();
    let os_name = sysinfo::get_os_name();
    let os_version = sysinfo::get_os_version();
    println!("cpu_limit: {:?}", cpu_limit);
    println!("memory_limit: {:?}", memory_limit);
    println!("hostname: {:?}", hostname);
    println!("os_name: {:?}", os_name);
    println!("os_version: {:?}", os_version);
    println!("--------------------------------");

    tokio::spawn(async move {
        let pid = ::sysinfo::get_current_pid();
        println!("pid: {:?}", pid);
        let mut big_map = vec![123456; 1000000];
        loop {
            big_map.extend(vec![123456; 10000]);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for _i in 0..10 {
        let pid = ::sysinfo::get_current_pid();
        println!("pid: {:?}", pid);
        let cpu_usage = sysinfo::get_cpu_usage();
        let sys_cpu_usage = sysinfo::cpu::get_cpu_usage();
        let memory_usage = sysinfo::get_memory_usage();
        let tcp_connections = sysinfo::get_tcp_connections();
        let tcp_conn_established =
            sysinfo::net::get_tcp_connections(Some(sysinfo::net::TcpConnState::Established));
        println!("cpu_usage: {:?}", cpu_usage);
        println!("sys_cpu_usage: {:?}", sys_cpu_usage);
        println!("memory_usage: {:?}", memory_usage);
        println!("tcp_connections: {:?}", tcp_connections);
        println!("tcp_conn_established: {:?}", tcp_conn_established);
        println!("--------------------------------");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
