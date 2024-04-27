use ch3_files_with_modes::{File, FileHandle, FileOpenMode, FileState};

fn main() {
    let f1 = File::new("f1.txt");

    let f2 = File::from_options(
        "f2.txt",
        FileState::Opened(FileOpenMode::Read),
        FileHandle::Handle(123),
    );

    let f3 = File::from_options(
        "f3.txt",
        FileState::Opened(FileOpenMode::Write),
        FileHandle::None,
    );

    let mut files = [f1, f2, f3];

    for f in &files {
        println!("{f}");
        println!();
    }

    // uh oh, disk failure
    for ref mut f in &mut files {
         if f.state == FileState::Opened(FileOpenMode::Read) {
            f.state = FileState::Error(String::from("disk read failure"));
            println!("{f}");    
            break;
         } 
        println!("{f}");
        println!();
    }
}
