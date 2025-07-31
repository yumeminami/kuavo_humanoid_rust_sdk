use rosrust::ros_info;
use std::time;

fn main() {
    // Fetch args that are not meant for rosrust
    let args: Vec<_> = rosrust::args();

    if args.len() != 3 {
        eprintln!("usage: client X Y");
        return;
    }

    let a = args[1].parse::<i64>().unwrap();
    let b = args[2].parse::<i64>().unwrap();

    // Initialize node
    rosrust::init("add_two_ints_client");

    // Wait ten seconds for the service to appear
    rosrust::wait_for_service("add_two_ints", Some(time::Duration::from_secs(10))).unwrap();

    // Create client for the service
    let client = rosrust::client::<rosrust_msg::roscpp_tutorials::TwoInts>("add_two_ints").unwrap();

    // Synchronous call that blocks the thread until a response is received
    ros_info!(
        "{} + {} = {}",
        a,
        b,
        client
            .req(&rosrust_msg::roscpp_tutorials::TwoIntsReq { a, b })
            .unwrap()
            .unwrap()
            .sum
    );

    // Asynchronous call that can be resolved later on
    let retval = client.req_async(rosrust_msg::roscpp_tutorials::TwoIntsReq { a, b });
    rosrust::ros_info!("{} + {} = {}", a, b, retval.read().unwrap().unwrap().sum);
}
