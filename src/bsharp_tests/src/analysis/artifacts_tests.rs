use analysis::artifacts::symbols::{SymbolIndex, SymbolKind};
use analysis::framework::ArtifactStore;

#[test]
fn artifact_store_insert_and_get_arc() {
    #[derive(Debug, PartialEq)]
    struct MyArtifact(u32);

    let store = ArtifactStore::new();
    store.insert(MyArtifact(7));

    let got = store.get::<MyArtifact>().expect("artifact missing");
    assert_eq!(got.as_ref(), &MyArtifact(7));

    // Cloned Arc should point to same data
    let got2 = store.get::<MyArtifact>().unwrap();
    assert!(std::ptr::eq(&*got, &*got2));
}

#[test]
fn symbol_index_allows_multiple_ids_per_name() {
    let mut idx = SymbolIndex::new();
    let id1 = idx.insert(
        "A",
        SymbolKind::Class,
        Some("Ns.A".into()),
        Some("f1.cs".into()),
        None,
        None,
    );
    let id2 = idx.insert(
        "A",
        SymbolKind::Class,
        Some("Ns2.A".into()),
        Some("f2.cs".into()),
        None,
        None,
    );
    assert_ne!(id1, id2);

    let ids = idx.get_ids_by_name("A").cloned().unwrap_or_default();
    assert_eq!(ids.len(), 2);
    let s1 = idx.get(ids[0]).unwrap();
    let s2 = idx.get(ids[1]).unwrap();
    assert_ne!(s1.fqn, s2.fqn);
}
