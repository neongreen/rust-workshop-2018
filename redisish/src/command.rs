#[derive(Debug, Clone)]
pub enum Command {
    /// Insert the specified value at the head of the list stored at key. If
    /// key does not exist, it is created as empty list before performing
    /// the push operation.
    ///
    /// Example: `PUBLISH numbers one`. Note the lack of quotes.
    Publish { list: String, value: String },

    /// Remove and return the first element of the list stored at key.
    ///
    /// Example: `RETRIEVE numbers`.
    Retrieve { list: String },
}
