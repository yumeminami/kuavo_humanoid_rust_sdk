fn main() {
    // Initialize node
    rosrust::init("param_test");

    // Create parameter, go through all methods, and delete it
    let param = rosrust::param("~foo").unwrap();
    rosrust::ros_info!("Handling ~foo:");
    rosrust::ros_info!("Exists? {:?}", param.exists()); // false
    param.set(&42u64).unwrap();
    rosrust::ros_info!("Get: {:?}", param.get::<u64>().unwrap());
    rosrust::ros_info!("Get raw: {:?}", param.get_raw().unwrap());
    rosrust::ros_info!("Search: {:?}", param.search().unwrap());
    rosrust::ros_info!("Exists? {}", param.exists().unwrap());
    param.delete().unwrap();
    rosrust::ros_info!("Get {:?}", param.get::<u64>().unwrap_err());
    rosrust::ros_info!(
        "Get with default: {:?}",
        param.get::<u64>().unwrap_or(44u64)
    );
    rosrust::ros_info!("Exists? {}", param.exists().unwrap());
}
