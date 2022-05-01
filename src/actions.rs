/// Actions that can be performed on an account.

/// Credits clients asset account.
const DEPOSIT: &str = "deposit";

/// Debit a clients account.
const WITHDRAWAL: &str = "withdrawal";

/// Dispute an erroneous transaction.
const DISPUTE: &str = "dispute";

/// Represents a resolution to a dispute.
const RESOLVE: &str = "resolve";

/// Chargeback represents a final state of reversing transaction.
/// Calling chargeback will result in an account being locked and no further actions can be performed
/// on that account.
const CHARGEBACK: &str = "chargeback";