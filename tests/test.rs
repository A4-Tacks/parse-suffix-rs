use std::{net::Ipv4Addr, path::PathBuf};

#[test]
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
