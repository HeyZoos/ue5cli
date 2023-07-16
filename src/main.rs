mod config;
mod manager;
mod my_format;
mod third_party_library_details;

fn main() {
    config::set("foo", "bar");
    dbg!(config::get("foo"));
    config::clear();
    config::set("key", "foo");
}
