# RustSimpleRestaurantApi

## How to install:

1. Git clone the repo to a machine with Rust + Cargo installed.
2. Open a terminal in the repo directory.
3. Start the server with `cargo run`.
4. Access at `localhost:7878`

---

## Endpoints:

1. `GET /`
2. `GET tables/<table_number>`
3. `GET tables/<table_number>/<order_item_ID>`
4. `POST tables/<table_number>/AddItem/<menu_item_ID>`
5. `DELETE tables/<table_number>/<order_item_ID>`

---

## Design considerations:

### Libraries

To fit with the spirit of the test, I opted to use as few external libraries as possible (Chrono and HTTParse). This was to show off what I know and can implement independently, rather than what library tutorials I can follow.

The exception to this was my starting point - the rust documentation's basic multithreaded server [here](https://doc.rust-lang.org/book/ch20-02-multithreaded.html). This was a good intro to rust and the strict memory management it enforces. It also provided a thread pool to allow the concurrent access to the api. If I were not to have used this, I would likely have needed to go for something like **Tokio** to achieve the multi-threading.

I also used **HTTParse** to handle parsing the incoming requests. A richer library like **Hyper** would have made this task easier, as HTTParse only splits the headers from the request method (GET, POST, ETC). However, the advantage of using HTTParse is that I could experiment with string processing in Rust.

I also had to implement a rudimentary web template, which using a library would have simplified massively. Something like angular would have made passing data to the front-end much more straight forward and improved separation of concerns (an area where my solution could definitely be improved).

### Data Structures

As the tables are the core entity that the orders hang from, while being relatively static themselves, it makes more sense to store them in a way that provides the fastest access. I chose to use a HashMap, as it's effectively the same as a dictionary in other languages, with constant time element access.

The orders could also be stored in a dictionary, as similarly they are nearly always directly accessed, however I don't forsee a table having a number of orders where you would get a tangible cost to iterating over a vector.

This could also be improved by having some kind of non-volatile data store, like a database.

### Concurrency

To store the tables (and their contained orders), I use Arc to give each thread a reference to the hashmap on the heap. I then used Mutex to ensure that only one thread can access the store at any time so that it can be used across threads safely.

### Virtual Clients

The virtual clients all run in their own threads, doing random web queries to mimic real clients. They trigger at random intervals, and perform GET, POST, and DELETE requests.

---

## Potential expansion/improvement

There are a number of ways this solution could be improved, for example:

- Use of libraries such as Hyper and Tokio.
- Use of a non-volatile data-store like an relational database.
- Separation of concerns through splitting the data storage, view, and "controller" into separate modules.
- Better handling of "panicing" methods. Using `unwrap` everywhere isn't ideal when you can handle the error more gracefully.
- Using a better HTTP library would allow a much richer api, probably opting to use the body or query params for providing data, rather than a `/<id>` approach.
- A structure that lends itself more to unit-test'ability. 
