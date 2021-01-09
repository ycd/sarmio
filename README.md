# Sarmio

### Distributed unique ID generator, inspired by [Twitter's snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake.html).


Sarmio creates a unique ID that is between the range of `0 > n > (2 ^ 64) -1`, also known as unsigned 64 bit integer.


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
    let mut sarmio_one = sarmio::Sarmio::new(255);
    let mut sarmio_two = sarmio::Sarmio::new(555);
    // Sarmio implements Iterator
    // Which means you can iterate over it to create new IDs.
    let id1 = match sarmio_one.next_id() {
        Some(s) => s,
        None => 0,
    };

    let id2 = match sarmio_two.next_id() {
        Some(s) => s,
        None => 0,
    };

    // Or create a new  with next_id() syntax.

    // Decompose it, get the values like
    // Unix time in that moment, machine id
    // and the Unique ID.
    let id1_decomposed = sarmio::decompose(id1);
    let id2_decomposed = sarmio::decompose(id2);

    println!("{:?}", id1_decomposed); // ID { id: 27015264398737663, machine_id: 255, time: 1610235238 }
    println!("{:?}", id2_decomposed); // ID { id: 27015264398737963, machine_id: 555, time: 1610235238 }

    // Check which ID is older.
    let is_older = id2_decomposed.older(&id1_decomposed);

    println!("{:?}", is_older); // false

    // Check whether the ID's are created in the same machine.
    let same_machine = id2_decomposed.same_machine(&id1_decomposed);

    println!("{:?}", same_machine) // false
}
```