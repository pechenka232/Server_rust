use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Сервер запущен заходи на http://127.0.0.1:7878");

    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("\nСловил подключение");
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Ошибка  {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    
    let buf_reader = BufReader::new(&mut stream);

    
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    
    if http_request.is_empty() {
        return;
    }

    
    println!("Входящий HTTP Запрос ");
    for line in &http_request {
        println!("{}", line);
    }
   

    
    let request_line = &http_request[0];
    
   
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 3 {
        eprintln!("Некорректный формат");
        return;
    }
    
    let method = parts[0];
    let uri = parts[1];
    let version = parts[2];

    println!("Лог Метод: {}, Путь: {}, Протокол: {}", method, uri, version);

    

    let response_body = "<h1>200 OK </h1>";
    let status_line = "HTTP/1.1 200 OK";
    let length = response_body.len();

    let response = format!(
        "{}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, response_body
    );

    
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Ошибка при отправке ответа: {}", e);
    }
    
   
    stream.flush().unwrap();
    
   
    println!("Соединение оборвано.");
}
// 7 дней практикyю Rust мой 1 пет проект
