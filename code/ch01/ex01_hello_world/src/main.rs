fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions {
        println!("{link}", link = &region);
    }
}

fn main() {
    greet_world();
}
