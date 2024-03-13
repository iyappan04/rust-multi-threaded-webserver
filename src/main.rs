use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main(){

    // fn main() -> std::io::Result<()> {
    //     let stream = Arc::new(TcpStream::connect("127.0.0.1:34254")?);
    //     let (r, w) = (Arc::clone(&stream), stream);
    
    //     let thr1 = std::thread::spawn(move || -> std::io::Result<()> {
    //         let r = BufReader::new(r.as_ref());
    //         for line in r.lines() {
    //             println!("received: {}", line?);
    //         }
    //         Ok(())
    //     });
    //     let thr2 = std::thread::spawn(move || {
    //         let mut w = BufWriter::new(w.as_ref());
    //         w.write_all(b"Hello\n")
    //     });
    //     thr1.join().unwrap()?;
    //     thr2.join().unwrap()?;
    //     Ok(())
    // }


    let listener = Arc::new(TcpListener::bind("127.0.0.1:7878").unwrap());

    // let (r, w) = (Arc::clone(&listener), listener);

    

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // let connection = Arc::new(Mutex::new(stream));

        // let thread1 = Arc::clone(&connection);
        // let thread2 = Arc::clone(&connection);
        
        thread::spawn(move || {
            handle_connection(stream);
            println!("From Thread 1");
        });
        
        // handle_connection(stream);

        // handle.join().unwrap();

        // let steamone = &stream;

        // thread::spawn(|| {
        //     handle_connection(stream);
        //     println!("From Thread 2");
        // });


    }
}


// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     println!("Request: {:#?}", http_request);

//     let status_line = "HTTP/1.1 200 OK";
//     let contents = fs::read_to_string("src/hello.html").unwrap();
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     if request_line == "GET /some HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = fs::read_to_string("src/hello.html").unwrap();
//         let length = contents.len();

//         let response = format!(
//             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//         );

//         stream.write_all(response.as_bytes()).unwrap();
//     } else if request_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = "hello";
//         let length = contents.len();

//         let response = format!(
//             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//         );

//         stream.write_all(response.as_bytes()).unwrap();
//     }
//     else {
//         println!("None");
//     }
// }


fn handle_connection(mut stream: TcpStream) {
    


    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(30));
            ("HTTP/1.1 200 OK", "src/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();


}