use std::{
    ffi::CStr,
    io::{stdin, stdout, Read, Write},
    net::{TcpListener, TcpStream},
    str::Utf8Error, 
    thread::spawn
};

use local_ip_address::local_ip;


fn main() {
    let mut menu = String::new();
    println!("Host는 Host 입력, Join는 그외 입력");
    stdin().read_line(&mut menu).unwrap();
    println!("");
    if menu.to_lowercase().trim() == "host"{
        hosting();
    }
    else{
        joining();
    }
}

fn hosting(){
    let mut port_check = true;
    let mut port = 0;

    while port_check {
        print!("Port: ");
        stdout().flush().unwrap();
        
        let mut input_port = String::new();
        stdin().read_line(&mut input_port).unwrap();
        let result_port = input_port.trim().parse::<i32>();
        match result_port {
            Ok(num) => {
                port = num;
                port_check = false;
            }
            Err(_) => {
                println!("숫자로 입력 하십시요");
            }
        }
    }
    let addr = format!("{}:{}",get_ip(),port);
    let listener = TcpListener::bind(addr.trim()).unwrap();
    println!("주소: {}",addr);

    for stream in listener.incoming(){
        if stream.is_ok(){
            let stream = stream.unwrap();
            println!("연결됨");
            
            let send_stream = stream.try_clone().unwrap();
            let read_stream = stream;

            spawn(move || {send(send_stream);});
            spawn(move || {read(read_stream, "Client".to_string());});

            loop{}
        }
    }

}

fn joining(){
    print!("IP: ");
    stdout().flush().unwrap();

    let mut ip = String::new();
    stdin().read_line(&mut ip).unwrap();

    let stream = TcpStream::connect(ip.trim()).unwrap();
    println!("연결됨");
            
    let send_stream = stream.try_clone().unwrap();
    let read_stream = stream;

    spawn(move || {send(send_stream);});
    spawn(move || {read(read_stream, "Host".to_string());});

    loop{}
}

fn buffer_to_string(buf: &[u8]) -> Result<&str, Utf8Error>{
    let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
    cstr.to_str()
}

fn send(mut stream: TcpStream){
    loop {
        let mut msg = String::new();
        stdin().read_line(&mut msg).unwrap();
        stream.write(msg.trim().as_bytes()).unwrap();
    }
}

fn read(mut stream: TcpStream,name: String){
    loop{
        let mut msg = [0; 1024];
        stream.read(&mut msg).unwrap();
        let text = buffer_to_string(&msg);
        println!("{}: {:?}",name,text.unwrap());
    }
}

fn get_ip() -> String{
    let ip = local_ip().unwrap();
    ip.to_string()
}