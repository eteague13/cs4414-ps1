//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;

static mut visitor_count: int = 0;


static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";

unsafe fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{

    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
			let first_split = request_str.find_str("/");
			let second_split = request_str.find_str("HTTP");
			let file_name = request_str.slice(first_split.unwrap()+1, second_split.unwrap()-1);
			let current_directory = std::os::getcwd();
			let file_path: Path = current_directory.push(file_name);
			
			let fileReader: Result<@Reader, ~str> = std::io::file_reader(&file_path);
			let mut file_lines: ~[~str];
			match fileReader {
				Ok(reader) => {file_lines = reader.read_lines();}
				Err(msg) => {file_lines = ~[];}//fail!("Cannot open file");}
			}
			
			visitor_count+=1;
                        println(fmt!("Request received:\n%sVisitor Requests: %? ", request_str, visitor_count));

			let mut vec_iterator = file_lines.iter();
			loop {
				match vec_iterator.next() {
					Some(pattern) => {println(fmt!("%s", *pattern));}
					None => {break;}
				}
			}
                        let response: ~str = ~
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { backgound-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             </body></html>\r\n";

                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
