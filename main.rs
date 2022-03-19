use std::io::Write;
use std::io;

fn main() {
    println!("Hello, how do you want to call your file ?");
    let mut filename = String::new().to_owned();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
     if filename.ends_with('\n') {
        filename.pop();
        if filename.ends_with('$') {
            filename.pop();
        }
    }
    let endchar: &str = ".html";
    let finalfile: String = filename + endchar;
    println!("What do you want your title to be ?");
    let mut title = String::new().to_owned();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    println!("What do you want your paragraph to be");
    let mut paragraph: String= String::new();
    io::stdin()
        .read_line(&mut paragraph)
        .expect("failed to read line");
    println!("Which Image do you want to insert? (Please use the url format)");
    let mut image: String= String::new();
    io::stdin()
        .read_line(&mut image)
        .expect("Failed to read line");
    println!("Which links do you want ?");
    let mut link: String= String::new();
    io::stdin()
        .read_line(&mut link)
        .expect("Failed to read line");
    let mut file = std::fs::File::create(finalfile).expect("create failed");
    file.write_all("<!DOCTYPE html>".as_bytes()).expect("write failed");
    file.write_all("\n<html>".as_bytes()).expect("write failed");
    file.write_all("\n<head>".as_bytes()).expect("write failed");
    file.write_all("\n<title></title>".as_bytes()).expect("write failed");
    file.write_all("\n<style></style>".as_bytes()).expect("write failed");
    file.write_all("\n</head>".as_bytes()).expect("write failed");
    file.write_all("\n<body>".as_bytes()).expect("write failed");
    file.write_all("\n<center><h1>".as_bytes()).expect("write failed");
    file.write_all(title.as_bytes()).expect("write failed");
    file.write_all("</h1></center>".as_bytes()).expect("write failed");
    file.write_all("\n<p>".as_bytes()).expect("write failed");
    file.write_all(paragraph.as_bytes()).expect("write failed");
    file.write_all("</p>".as_bytes()).expect("write failed");
    file.write_all("<center><img src ='".as_bytes()).expect("write failed");
    file.write_all(image.as_bytes()).expect("write failed");
    file.write_all("'>".as_bytes()).expect("write failed");
    file.write_all("</img></center>".as_bytes()).expect("write failed");
    file.write_all("\n<li>".as_bytes()).expect("write failed");
    file.write_all(link.as_bytes()).expect("write failed");
    file.write_all("</li>".as_bytes()).expect("failed to write line");
    file.write_all("\n </body>".as_bytes()).expect("write failed");
    file.write_all("\n </html>".as_bytes()).expect("write failed");
    println!("data written to file" );
}
