mod iflet;
mod matching;

fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home = {:?}, Loopback = {:?}", home, loopback);

    println!("-----------------------------------------------");
    matching::test();

    println!("-----------------------------------------------");
    iflet::test();
}
