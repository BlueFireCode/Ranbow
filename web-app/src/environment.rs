pub fn ranbow_host_url() -> String {
    let var = std::option_env!("RANBOW_HOST_URL").unwrap_or("http://localhost:8080");
    String::from(var)
}