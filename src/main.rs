use nostr_sdk::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    let keys = Keys::generate();
    let _ = Client::new(&keys);
}
