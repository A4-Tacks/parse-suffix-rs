Process the string suffix as `.parse::<suffix>().unwrap()`

# Examples
```rust
use std::{net::Ipv4Addr, path::PathBuf};

#[parse_suffix::parse_string_suffix]
fn test() {
    assert_eq!("23"i32, 23);
    assert_eq!("23"PathBuf, PathBuf::from("23"));
    assert_eq!("true"bool, true);
    assert_eq!("false"bool, false);
    assert_eq!("192.168.1.1"Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
    assert_eq!(r"192.168.1.1"Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
    assert_eq!(r#"192.168.1.1"#Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
}
```
Expand to:
```rust
use std::{net::Ipv4Addr, path::PathBuf};

fn test() {
    assert_eq("23".parse::<i32>().unwrap(), 23);
    assert_eq("23".parse::<PathBuf>().unwrap(), PathBuf::from("23"));
    assert_eq("true".parse::<bool>().unwrap(), true);
    assert_eq("false".parse::<bool>().unwrap(), false);
    assert_eq("192.168.1.1".parse::<Ipv4Addr>().unwrap(), Ipv4Addr::new(192, 168, 1, 1));
    assert_eq(r"192.168.1.1".parse::<Ipv4Addr>().unwrap(), Ipv4Addr::new(192, 168, 1, 1));
    assert_eq(r#"192.168.1.1"#.parse::<Ipv4Addr>().unwrap(), Ipv4Addr::new(192, 168, 1, 1));
}
```
