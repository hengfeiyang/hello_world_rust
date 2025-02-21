mod db;
mod sysinfo;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cpu_num = sysinfo::cpu::get_cpu_num();
    let cpu_usage = sysinfo::cpu::get_cpu_usage();
    let memory_total = sysinfo::mem::get_total_memory();
    let memory_used = sysinfo::mem::get_memory_usage();
    let tcp_connections = sysinfo::net::get_tcp_connection_num(None);
    let tcp_conn_established =
        sysinfo::net::get_tcp_connection_num(Some(sysinfo::net::TcpConnState::Established));

    println!("cpu_num: {:?}", cpu_num);
    println!("cpu_usage: {:?}", cpu_usage);
    println!("memory_total: {:?}", memory_total);
    println!("memory_used: {:?}", memory_used);
    println!("tcp_connections: {:?}", tcp_connections);
    println!("tcp_conn_established: {:?}", tcp_conn_established);

    Ok(())
}
