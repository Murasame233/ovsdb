use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use super::Params;

/// OVSDB operation to be performed.  Somewhat analgous to a SQL statement.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "op")]
pub enum Operation {
    /// An OVSDB `select` operation
    #[serde(rename = "select")]
    Select {
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// A collection of clauses to act as filters against the table data.
        #[serde(rename = "where")]
        clauses: Vec<String>,
        /// Which columns you want to select
        columns: Vec<String>,
    },
    /// `insert` operation
    #[serde(rename = "insert")]
    Insert {
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// The row you want to insert.
        row: String,
        /// UUID
        #[serde(rename = "uuid-name")]
        uuid_name: Option<String>,
    },
    /// `update` operation
    #[serde(rename = "update")]
    Update {
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// Condition
        #[serde(rename = "where")]
        clauses: Vec<String>,
        /// The row you want to update.
        row: Option<String>,
    },
    /// `mutate` operation
    #[serde(rename = "mutate")]
    Mutate {
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// Condition
        #[serde(rename = "where")]
        clauses: Vec<String>,
        /// The edit you want to apply.
        mutations: Option<String>,
    },
    /// `delete` operation
    #[serde(rename = "delete")]
    Delete{
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// A collection of clauses to act as filters against the table data.
        #[serde(rename = "where")]
        clauses: Vec<String>,
    },
        /// `wait` operation
    #[serde(rename = "wait")]
    Wait{
        /// the Timeout
        timeout: Option<u64>,
        /// The [Table][crate::schema::Table] to operate against.
        table: String,
        /// A collection of clauses to act as filters against the table data.
        #[serde(rename = "where")]
        clauses: Vec<String>,
        /// The columns you want to wait
        columns: Vec<String>,
        /// `!=`or`==`
        until: String,
        ///c the rows you want to wait
        rows: Vec<String>
    },
    /// `commit` operation
    #[serde(rename = "commit")]
    Commit{
        /// if its true, all transaction will be store to disk/durable, then return.
        durable: bool
    },
    /// `abort` operation
    #[serde(rename = "abort")]
    Abort{

    },
    /// `comment` operation
    #[serde(rename = "comment")]
    Comment{
        /// comment a transaction
        comment: String
    },
    /// `assert` operation
    #[serde(rename = "assert")]
    Assert{
        /// will be lock if clients does not own the lock;
        lock: String

    }
}

/// Parameters for the `transact` OVSDB method.
#[derive(Debug, Deserialize)]
pub struct TransactParams {
    database: String,
    operations: Vec<Operation>,
}

impl TransactParams {
    /// Create a new set of `transact` parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ovsdb::protocol::method::{Operation, TransactParams};
    ///
    /// let op = Operation::Select { table: "Bridges".into(), clauses: vec![] };
    /// let params = TransactParams::new("Bridges", vec![op]);
    /// ```
    pub fn new<T>(database: T, operations: Vec<Operation>) -> Self
    where
        T: Into<String>,
    {
        Self {
            database: database.into(),
            operations,
        }
    }
}

impl Params for TransactParams {}

impl Serialize for TransactParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.operations.len() + 1))?;
        seq.serialize_element(&self.database)?;
        for op in &self.operations {
            seq.serialize_element(&op)?;
        }
        seq.end()
    }
}
