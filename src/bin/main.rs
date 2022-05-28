#[macro_use]
extern crate lazy_static;
use chrono::{DateTime, Duration, Utc};
use server::ThreadPool;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread;

static ORDER_COUNT: AtomicU64 = AtomicU64::new(0);
lazy_static! {
    static ref TABLES: Mutex<HashMap<u64, Table>> = Mutex::new(HashMap::new());
}
struct OrderItem {
    id: u64,
    table_number: u64,
    menu_reference: u64,
    order_time: DateTime<Utc>,
    cooking_time: Duration,
}

impl OrderItem {
    pub fn new(table_number: u64, menu_reference: u64, cooking_time: Duration) -> Self {
        Self {
            id: (ORDER_COUNT.fetch_add(1, Ordering::SeqCst)),
            table_number: (table_number),
            menu_reference: (menu_reference),
            order_time: (Utc::now()),
            cooking_time: (cooking_time),
        }
    }
}

struct Table {
    table_number: u64,
    orders: Vec<OrderItem>,
}

impl Table {
    pub fn new(table_number: u64) -> Self {
        Self {
            table_number: (table_number),
            orders: Vec::with_capacity(100),
        }
    }

    pub fn add_order(&mut self, order: OrderItem) {
        self.orders.push(order);
    }

    pub fn delete_order(&mut self, order_item_id: u64) {
        let index = self
            .orders
            .iter()
            .position(|r| r.id == order_item_id)
            .unwrap();
        self.orders.remove(index);
    }

    pub fn get_order(&mut self, order_item_id: u64) -> &OrderItem {
        let index = self
            .orders
            .iter()
            .position(|r| r.id == order_item_id)
            .unwrap();
        return self.orders.get(index).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for i in 0..100 {
        TABLES.lock().unwrap().insert(i, Table::new(i));
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = handle_path(buffer);

    let contents = fs::read_to_string(filename).unwrap();

    let response: String = build_response_string(status_line.to_string(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn build_response_string(status_line: String, contents: String) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    )
}

fn handle_path(buffer: [u8; 1024]) -> (&'static str, &'static str) {
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    match req.path {
        Some(ref path) => {
            println!("Path: {}", path);
            let split = path.split_inclusive("/");
            let parts: Vec<&str> = split.collect();

            if "tables/".eq_ignore_ascii_case(parts.get(1).unwrap()) {
                if parts.len() == 2 {
                    // GET tables/     - all items for all tables
                    ("HTTP/1.1 200 OK", "index.html")
                } else if parts.len() == 3 {
                    // POST tables/#No     - add item(s) to table (json blob in body)
                    // GET tables/#No      - list items for table
                    let table_number_str = parts.get(2).unwrap().strip_suffix("/").unwrap();
                    let table_number = table_number_str.to_string().parse::<u64>().unwrap();

                    ("HTTP/1.1 200 OK", "index.html")
                } else if parts.len() == 4 {
                    // DELETE tables/#No/OrderItemID   - delete matching order
                    // GET tables/#No/OrderItemID     - details about a specific order
                    let table_number_str = parts.get(2).unwrap().strip_suffix("/").unwrap();
                    let table_number = table_number_str.to_string().parse::<u64>().unwrap();

                    let order_item_id_str = parts.get(3).unwrap().strip_suffix("/").unwrap();
                    let order_item_id = order_item_id_str.to_string().parse::<u64>().unwrap();
                    ("HTTP/1.1 200 OK", "index.html")
                } else {
                    ("HTTP/1.1 404 NOT FOUND", "404.html")
                }
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            }
        }
        None => ("HTTP/1.1 200 OK", "index.html"),
    }
}
