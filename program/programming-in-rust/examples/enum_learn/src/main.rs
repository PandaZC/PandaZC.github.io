#[derive(Debug)]
// enum IpAddrType {
//     IPv4,
//     IPv6,
// }

// struct IpAddrInfo {
//     ip_type: IpAddrType,
//     addr: String,
// }

// fn main() {
//     let route = IpAddrInfo {
//         ip_type: IpAddrType::IPv4,
//         addr: String::from("127.0.0.1"),
//     };

//     print!("{:?} {}", route.ip_type, route.addr);
// }

// enum IpAddr {
//     IPv4(u8, u8, u8, u8),
//     IPv6(String),
// }

// fn main() {
//     let ipv4 = IpAddr::IPv4(127, 0, 0, 1);
//     let ipv6 = IpAddr::IPv6(String::from("AA22:BB11:1122:CDEF:1234:AA99:7654:7410"));

//     println!("ipv4: {:?}", ipv4);
//     println!("ipv6: {:?}", ipv6);
// }

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(coin));
}