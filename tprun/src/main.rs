mod db;
mod sysinfo;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cpu_limit = sysinfo::get_cpu_limit();
    let memory_limit = sysinfo::get_memory_limit();
    println!("cpu_limit: {:?}", cpu_limit);
    println!("memory_limit: {:?}", memory_limit);

    for _i in 0..10 {
        let cpu_usage = sysinfo::get_cpu_usage();
        let memory_usage = sysinfo::get_memory_usage();
        let tcp_connections = sysinfo::get_tcp_connections();
        let tcp_conn_established =
            sysinfo::net::get_tcp_connections(Some(sysinfo::net::TcpConnState::Established));
        println!("cpu_usage: {:?}", cpu_usage);
        println!("memory_usage: {:?}", memory_usage);
        println!("tcp_connections: {:?}", tcp_connections);
        println!("tcp_conn_established: {:?}", tcp_conn_established);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
