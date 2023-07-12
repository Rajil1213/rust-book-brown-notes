pub fn test() {
    let mut hello = String::from("hello ");
    let world = "world";
    hello.push_str(world);

    println!("hello.push_str({world}) = {hello}");

    let exclaim = '!';
    hello.push(exclaim);

    println!("(hello {world}).push({exclaim}) = {hello}");

    let hello = String::from("hello, ");
    let world = String::from("world");
    let hello_world = hello + &world;
    println!("hello, + {world} = {hello_world}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");

    println!("{tic}-{tac}-{toe} = {tic_tac_toe}");

    // UTF-8 demo:
    let namaste = "नमस्ते";
    let namaste_bytes = namaste.as_bytes();
    println!("Bytes in {namaste} = {namaste_bytes:?}");

    let namaste_as_chars = namaste.chars();
    let mut namaste_scalars: Vec<char> = Vec::new();
    for scalar in namaste_as_chars {
        namaste_scalars.push(scalar);
    }
    println!("Scalars (chars) in {namaste} = {:?}", namaste_scalars);

    // string slice
    // let s = &namaste[1..3];
    // this panics at runtime
    // println!("slice of namaste = {:?}", s);
}
