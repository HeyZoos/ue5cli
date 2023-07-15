mod config;

fn main() {
    config::set("foo", "bar");
    dbg!(config::get("foo"));
    config::clear();
    config::set("key", "foo");
}
