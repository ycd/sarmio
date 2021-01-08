# Sarmio

### Distributed unique ID generator, inspired by [Twitter's snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake.html).


Sarmio creates a unique ID that is between the range of `0 > n > (2 ^ 63) -1`, also known as unsigned 64 bit integer.

It uses this equation to create a unique ID. 

```math
UNIX Epoch time: 32-bit unsigned integer 0 > n > 2147483647  
Machine ID: 8 bit unsigned integer. 0 > n > 256
```


## Usage

First of all, add `sarmio` as a dependency to your `cargo.toml`.

```toml
[dependencies]
sarmio = "0.1"
```



## Example

```rust
fn main() {
    // Create new Sarmio instance with a machine-id of 255.
    let mut s = sarmio::Sarmio::new(255);
    
    // Sarmio implements Iterator
    // Which means you can iterate over it to create new IDs.
    let v = match s.next() {
        Some(s) => s,
        None => None,
    };
    println!("{}", v);      // 18190711796065697536 

    // Or create a new  with next_id() syntax.

    
    // Decompose it, get the values like
    // Unix time in that moment, machine id
    // and the Unique ID.
    let p = sarmio::decompose(v);  

    println!("{:?}", p);    // {"id": 18190711796065697536, "time": 1610671519, "machine-id": 255}
}
```