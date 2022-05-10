### Run

To run the application use `cargo run -- <your_test_data.csv>` e.g. `cargo run -- ./tests/scenarios/deposits.csv`

### Tests

The scenarios' folder contains a series of files with different scenarios to tests the behaviour of the system. Unit tests are also included.

To run the tests use `cargo test`

### Assumptions

1. The input file only contains valid transactions types Deposit, Withdrawal, Dispute, Resolve and Chargeback
2. Once a chargeback has occurred an account is locked and no more actions can be performed on that account. For example, 
   making a deposit after a chargeback has occurred will result in an `AccountLockedError` and the deposit will not be recognised
3. Deposits are the only type that can be disputed
4. A transaction can be disputed multiple times as long as it has been resolved before being re disputed

### Design
* Account domain object encapsulates transaction data and the operations on that data. I chose to have the transaction as part of the Account as it expressed a stronger relationship between the account and the transaction opposed to tx_engine hold the accounts and transactions in maps.
* A common pattern when dealing with accounts and transactions is to use event sourcing, an event store and commands with a domain aggregate this gives the added benefit of having a complete transaction history and being able to rebuild an account to any state in time. However, this was not in the spec, and it would require more space to store the necessary info.
* From the spec we can derive that only deposits can be disputed therefore the transaction type has not been recorded with the transaction to save of memory usage. For event sourcing the type would be required to build the state of the aggregate for any given time
* As the spec only poses the question about multiple streams rather than specifies this as a requirement this has not been coded. To support multiple streams two approaches can be used either mutex or provide a channel (for example, crossbeam channels) for the csv parser to send messages and then a handler can process them with the domain object.