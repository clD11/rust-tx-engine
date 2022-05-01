### Run

``
cargo run -- <input-filename.csv> > <output-fimename.csv>
``

### Assumptions

1. The input file only contains valid transactions types Deposit, Withdrawal, Dispute, Resolve and Chargeback
2. Once a chargeback has occurred an account is locked and no more actions can be performed on that account. For example, 
   making a deposit after a chargeback has occurred will result in an `AccountLockedError` and the deposit will not be recognised
3. The code will be run in a single thread. For multiple input streams use mutex or channel implementations (_see concurrent module_)