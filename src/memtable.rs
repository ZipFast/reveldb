/// MemTable is a in-memory data struct to keep the key-value pairs sorted,
/// when the size of MemTable grow to a pre-determined threshold, the contents
/// of MemTable is flushed to the SSTable file.
///
/// MemTables are reference counted. The initial reference count is zero and
/// the caller must call Ref() at least once.
pub struct MemTable {}
