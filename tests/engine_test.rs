use fluffdb::Engine;
use std::fs;

#[test]
fn test_engine_persistence() {
    let db_path = "test_wal.log";
    let _ = fs::remove_file(db_path);

    {
        let mut db = Engine::open(db_path).unwrap();
        db.put(b"hello", b"world").unwrap();
        db.put(b"foo", b"bar").unwrap();
    }

    {
        let db = Engine::open(db_path).unwrap();
        assert_eq!(db.get(b"hello"), Some(b"world".to_vec()));
        assert_eq!(db.get(b"foo"), Some(b"bar".to_vec()));
        assert_eq!(db.get(b"nonexistent"), None);
    }

    let _ = fs::remove_file(db_path);
}
