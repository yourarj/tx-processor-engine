# Awesome TXpro Engine 
An Awesome 😎 tx processor engine

## Assumptions:

- Once disputed transaction is `RESOLVED`/`CHARGEBACKED` it can't be `DISPUTE`d again
- Only `DEPOSIT` type transaction can be marked as disputed

## How To
to run application run `cargo run -- <path_to_csv_file>` output will be written to console

for more detailed information run `cargo run -- --help`

```
tx-processor-engine 0.4.0
Shinde Arjun
An awesome 😎 transaction processing engine

USAGE:
    tx_executor [OPTIONS] <FILE>

ARGS:
    <FILE>    file to process

OPTIONS:
    -h, --help          Print help information
    -l, --log-errors    should log errors?
    -V, --version       Print version information
```

## Use cases covered
covered cases can be found [here ↗](test_input/desc.md)

Probability tree considered while engineering the solution

![image description](resources/probability_tree.png)