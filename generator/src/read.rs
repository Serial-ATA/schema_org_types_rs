use std::io::Cursor;

use oxigraph::store::Store;

use crate::read::patches::insert_quads;

mod patches;

pub async fn read() -> Store {
	let store = Store::new().unwrap();
	let rdf = reqwest::get("https://schema.org/version/latest/schemaorg-all-https.rdf")
		.await
		.unwrap()
		.text()
		.await
		.unwrap();
	store
		.bulk_loader()
		.load_from_reader(oxigraph::io::RdfFormat::RdfXml, Cursor::new(rdf.as_bytes()))
		.unwrap();
	for insert_quad in insert_quads() {
		store.insert(insert_quad).unwrap();
	}
	store
}
