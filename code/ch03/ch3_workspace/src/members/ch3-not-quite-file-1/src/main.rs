use ch3_not_quite_file_1::{close, open, read, File};

fn main() {
    let mut f1 = File::from("f1.txt"); // <7> With the type declaration at line 3, `File` "inherits" all of String's methods 
    open(&mut f1);
    let is_read_imlemented = !true;
    if is_read_imlemented {
        read(&mut f1,&mut vec![]);
    }
    
    //read(f1 , vec![]); // <8> There's little point in calling this method
    close(&mut f1);
}