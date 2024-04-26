fn main() {
    let mut xyz:(u32,u32,u32)=(0,0,0);
    'ounter: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    xyz = (x,y,z);
                    break 'ounter;
                }
            }
        }
    }
    println!("x = {}, y = {}, z = {}", xyz.0,xyz.1,xyz.2);
}
