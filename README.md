# Awesome TXpro Engine 
An Awesome 😎 tx processor engine

## Assumptions:

- Once disputed transaction is `RESOLVED`/`CHARGEBACKED` it can't be `DISPUTE`d again
- Only `DEPOSIT` type transaction can be marked as disputed
- `DEPOSIT` / `WITHDRAWAL` of unrealistic amount (overflowing) are ignored. Has to malicious.
- `DEPOSIT` causing overflow are also ignores. Has to be malicious
- Any client with not a SINGLE valid transaction will be omitted from result.
## How To
 - Get help with  `cargo run -- --help`

 - Run with  `cargo run -- <path_to_csv_file>` 

 - Run with error to std err `cargo run -- -l <path_to_csv_file>`

 - Get app version with  `cargo run -- -V`

### help preview
```
tx-processor-engine 1.0.0
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


## Tests
to run all **43** tests, please run `cargo test`

## Design considerations
Following probability tree was considered while engineering the solution

![image description](resources/probability_tree.png)


## Use cases covered

### Description of cases covered
> in depth description of following cases be found [here ↗](test_input/desc.md)

The following flows have been covered
1. genesis -> deposit  
1. genesis -> withdraw
1. genesis -> dispute
1. genesis -> chargeback
1. genesis -> resolve
1. genesis -> deposit -> withdraw
1. genesis -> deposit -> dispute
1. genesis -> deposit -> resolve
1. genesis -> deposit -> chargeback
1. genesis -> deposit -> withdraw -> dispute
1. genesis -> deposit -> dispute -> dispute
1. genesis -> deposit -> dispute -> resolve
1. genesis -> deposit -> dispute -> chargeback
1. genesis -> deposit -> dispute -> withdraw
1. genesis -> deposit -> dispute -> resolve -> dispute
1. genesis -> deposit -> dispute -> resolve -> resolve
1. genesis -> deposit -> dispute -> resolve -> chargeback
1. genesis -> deposit -> dispute -> chargeback -> chargeback
1. genesis -> deposit -> dispute -> chargeback -> deposit
1. genesis -> deposit -> dispute -> chargeback -> withdraw
1. genesis -> deposit -> dispute -> chargeback -> dispute
1. genesis -> deposit -> partial_withdraw -> full_withdraw
1. deposit negative amount
1. withdraw negative amount
1. twice deposit amount that is max allowed by container type
1. deposit amount larger than max allowed by container type
1. deposit without amount
1. withdraw without amount
1. deposit without client id
1. deposit tx id
1. withdraw without client id
1. withdraw without tx id
1. dispute without client id
1. dispute without tx id
1. resolve without client id
1. resolve without tx id
1. chargeback without client id
1. chargeback without tx id
1. withdraw amount exactly f64::MAX
1. try negative number as client id
1. try client id larger than u16
1. try negative number as tx id
1. try tx id larger than u32 