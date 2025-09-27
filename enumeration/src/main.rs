#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn print(&self) {
        println!("IpAddr({:?}, {})", self.kind, self.address);
    }
}

// #[derive(Debug)]
// enum IpAddrSimple {
//     V4(String),
//     V6(String),
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    home.print();
    loopback.print();
    // let home_simple = IpAddrSimple::V4(String::from("127.0.0.1"));
    // let loopback_simple = IpAddrSimple::V6(String::from("::1"));
    // println!("home_simple={:?}, loopback_simple={:?}",
    //     home_simple, loopback_simple);
}

fn route(ip_type: IpAddrKind) {
    println!("IpAddrKind {:?}", ip_type);
}