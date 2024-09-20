#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_ipv4_pass() {
        for ip in &["127.0.0.1", "1.2.3.4", "123.4.56.7"] {
            assert!(ip.parse::<Ipv4Addr>().is_ok());
        }
    }

    #[test]
    fn test_ipv4_fail() {
        for ip in &[
            "",
            "1",
            "abc",
            "a.b.c.d",
            "01.102.103.104",
            "::ffff:192.0.2.128",
            " 123.4.56.7",
            "123.4.56.7 ",
            "123.4.56",
            "10002.3.4",
            "1.2.3.4.5",
            "256.0.0.0",
            "260.0.0.0",
        ] {
            assert!(ip.parse::<Ipv4Addr>().is_err());
        }
    }

    #[test]
    fn test_ipv6_pass() {
        for ip in &[
            "2001:db8:3333:4444:5555:6666:7777:8888",
            "2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF",
            "::",
            "2001:db8::",
            "::1234:5678",
            "2001:db8::1234:5678",
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "::ffff:192.0.2.128",
            "1:2:3:4:5:6:77:88",
        ] {
            assert!(ip.parse::<Ipv6Addr>().is_ok());
        }
    }

    #[test]
    fn test_ipv6_fail() {
        for ip in &[
            "",
            "1",
            "abc",
            "123.4.56.7",
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334 ",
            " 2001:db8::",
            "1:2:3:4:5:6:7:8:9",
            "1:2:3:4:5:6::7:8",
            ":1:2:3:4:5:6:7:8",
            "1:2:3:4:5:6:7:8:",
            "::1:2:3:4:5:6:7:8",
            "1:2:3:4:5:6:7:8::",
            "1:2:3:4:5:6:7:88888",
            // current regex rejects this, but std::net allows it
            // "2001:db8:3:4:5::192.0.2.33",
            "fe08::7:8%",
            "fe08::7:8i",
            "fe08::7:8interface",
        ] {
            let result = ip.parse::<Ipv6Addr>().is_err();
            assert!(result, "Expected error for IP: {}", ip);
        }
    }
}

fn main() {
    println!("Hello, world! This project is tests only.")
}
