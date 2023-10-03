use safe_drive::{
    context::Context, error::DynError, msg::common_interfaces::std_msgs
};

use std::fs::File;
use std::io::Write;

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("write_file_node", None, Default::default())?;

    // Create a subscriber.
    let subscriber = node.create_subscriber::<std_msgs::msg::String>("my_topic", None)?;

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    let mut file = File::create("./files/test.txt").unwrap();

    // Add a callback function.
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            let _ = file.write_all(msg.data.get_string().as_bytes());
        }),
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}