use clearscreen;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process;
fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn open(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}

struct AFile {
    path: String,
    contents: String,
}

impl AFile {
    fn set_contents(&mut self, new_contents: String) {
        let mut file =
            File::create(&self.path).expect("Failed to open file for writing"); // Use &self.path instead of cloning
        file.write_all(new_contents.as_bytes())
            .expect("Failed to write to file");
    }

    fn create_file(path: String) -> AFile {
        fs::File::create(&path).expect("Unable to create file");
        Self::new_file(path)
    }

    fn new_file(path: String) -> AFile {
        AFile {
            path,
            contents: String::new(),
        }
    }

    fn get_contents_of_file(file_struct: &mut Self) {
        let contents = open(&file_struct.path); // Pass a reference to the path string
        file_struct.contents = contents;
    }

    fn delete_file(&self) {
        fs::remove_file(&self.path).expect("Failed to remove file");
    }
}

fn main() {
    let choice = input("Enter choice: ");
    match choice.as_str() {
      "New" => {
          let path = input("Enter filename: ");
          AFile::create_file(path);
          println!("File created.");
      }

      "Edit" => {
          let file_name = input("Enter the file name: ");
          let mut curr_file = AFile::new_file(file_name);
          let new_contents = input("Enter new content: ");
          AFile::set_contents(&mut curr_file, new_contents);
          println!("File overwritten.")
      }

      "Read" =>{
        let file_name = input("Enter the file name: ");
        let mut curr_file = AFile::new_file(file_name);
        AFile::get_contents_of_file(&mut curr_file);
        println!("Contents:\n{}",curr_file.contents);
        
        
      }
      "Delete" => {
        let file_name = input("Enter the file name: ");
        let curr_file = AFile::new_file(file_name);
        AFile::delete_file(&curr_file);
      }

      "Exit" =>{
        println!("Exiting...");
        process::exit(0x0000);
      }
      
      _ => {
          println!("Invalid choice:
          Available options:
          Help - Displays this menu
          Read - Outputs the contents of the file
          Edit - Changes the contents of the file
          New - Creates a new file
          Delete - Removes a file
          Exit - Exits the program");
          main();
      }
    };
  clearscreen::clear().expect("Failed to clear screen");
  main();
}
