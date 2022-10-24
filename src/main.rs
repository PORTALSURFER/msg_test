mod channels;
mod dynamic;
mod static_pass;

use channels::channels;
use dynamic::dynamic_pass;
use static_pass::static_pass;

fn main() {
    // dynamic_pass();
    // static_pass();
    channels();
}
