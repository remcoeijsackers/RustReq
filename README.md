# RustReq
Example of how to handle requests in rustlang.
#### Prerequesets
* [`Rust`](https://www.rust-lang.org/tools/install)
* [`Cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### installation
```
cargo build
```

#### Usage
call get on a url 
```
cargo run get https://api.domain.com
```
or
```
./target/debug/rustreq get https://api.domain.com
```

call post on a url 
```
cargo run post https://api.domain.com {body}
```
or
```
./target/debug/rustreq post https://api.domain.com {body}
```