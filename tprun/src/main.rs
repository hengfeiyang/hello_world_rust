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

    let cpu_limit = sysinfo::cgroup::get_cpu_limit();
    let memory_limit = sysinfo::cgroup::get_memory_limit();

    println!("cgroup cpu_limit: {:?}", cpu_limit);
    println!("cgroup memory_limit: {:?}", memory_limit);

    let process_cpu_usage = sysinfo::cpu::get_process_cpu_usage();
    let process_memory_usage = sysinfo::mem::get_process_memory_usage();

    println!("process cpu_usage: {:?}", process_cpu_usage);
    println!("process memory_usage: {:?}", process_memory_usage);

    for i in 0..10 {
        let memory_usage_v1 = sysinfo::mem::get_process_memory_usage();
        let memory_usage_v2 = sysinfo::mem::get_memory_usage_from_memory_stats();
        println!("memory_usage_v1: {:?}", memory_usage_v1);
        println!("memory_usage_v2: {:?}", memory_usage_v2);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
