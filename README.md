# C01
```
> cargo run -p c01 -- target/
```

# C02
There are a number of issues in this little crate related to unsafety and soundness.

Please identify as many as you can and suggest how they can be fixed.

# C03
Prepare testing data:
```
> python3 gen.py
```

Build the rust solution
```
cargo build --release
```

Time both
```
time ../../target/release/c03
time python3 count_a.py
```

Rust promises to be a performant language yet the Rust impl is way slower than the python one.
