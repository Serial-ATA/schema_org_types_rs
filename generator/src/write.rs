use std::{
	collections::{BinaryHeap, HashMap},
	fmt::Write,
	path::{Path, PathBuf},
	str::FromStr,
	sync::{Arc, Mutex, MutexGuard},
};

use derivative::Derivative;
use indicatif::{MultiProgress, ProgressBar};
use oxigraph::store::Store;
use quote::{
	__private::{Span, TokenStream},
	ToTokens, quote,
};
use rayon::prelude::*;
use syn::LitStr;

use crate::{
	schema::{
		Schema, class::Class, data_type::DataType, enumeration::Enumeration, property::Property,
	},
	sparql::{SchemaQueries, SchemaQuerySolution, node_type::NodeType},
};

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct SchemaModuleInfo {
	pub mod_name: String,
	pub ty_name: String,
}

impl<T: Schema> From<&T> for SchemaModuleInfo {
	fn from(value: &T) -> Self {
		Self {
			mod_name: value.module_name(),
			ty_name: value.name().clone(),
		}
	}
}

trait ToModuleString {
	fn to_module_string(&self) -> String;
}
impl ToModuleString for &[SchemaModuleInfo] {
	fn to_module_string(&self) -> String {
		let schema_mods_and_pub_uses = self.iter().map(|schema| {
			let module_name = TokenStream::from_str(&format!("r#{}", schema.mod_name)).unwrap();
			let feature_name = LitStr::new(&schema.ty_name, Span::call_site());
			quote!(
				#[cfg(feature = #feature_name)]
				mod #module_name;
				#[cfg(feature = #feature_name)]
				pub use self::#module_name::*;
			)
		});
		quote!(
			use super::*;
			#(#schema_mods_and_pub_uses)*
		)
		.to_string()
	}
}

type FeatureMap = Arc<Mutex<HashMap<String, Vec<String>>>>;

trait WriteModules {
	fn write_module(&self, schemas_dir: &Path, feature_set: FeatureMap);
	fn write_parent_module(schemas: &[SchemaModuleInfo], schemas_dir: &Path)
	where
		Self: Sized;
	fn write_parent_module_folder(schemas_dir: &Path);
}

trait HandleWrite {
	fn handle_write(
		store: &Store,
		solution: &SchemaQuerySolution,
		schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
		schemas_dir: &Path,
		feature_set: FeatureMap,
	);
}

impl<T: Schema + ToTokens> HandleWrite for T {
	fn handle_write(
		store: &Store,
		solution: &SchemaQuerySolution,
		mut schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
		schemas_dir: &Path,
		feature_set: FeatureMap,
	) {
		let schema = Self::from_solution(store, solution);
		schema_module_infos.push(SchemaModuleInfo::from(&schema));
		schema.write_module(schemas_dir, feature_set);
	}
}

/// This function exists to make the futures below [`Send`].
///
/// Reference: <https://users.rust-lang.org/t/future-is-not-send-as-this-value-is-used-across-an-await-but-i-drop-the-value-before-the-await/57574/5>
fn pretty_please(str: &str) -> String {
	let syntax_tree = syn::parse_file(str).unwrap();
	prettyplease::unparse(&syntax_tree)
}

impl<T: Schema + ToTokens> WriteModules for T {
	fn write_module(&self, schemas_dir: &Path, feature_set: FeatureMap) {
		let mut file_path = PathBuf::from(&schemas_dir);
		file_path.push(Self::parent_module_name());
		file_path.push(format!("{}.rs", self.module_name()));
		std::fs::write(
			file_path,
			pretty_please(&self.to_token_stream().to_string()),
		)
		.unwrap();

		feature_set
			.lock()
			.unwrap()
			.insert(self.name().to_string(), self.dependencies());
	}

	fn write_parent_module(schema_module_infos: &[SchemaModuleInfo], schemas_dir: &Path) {
		let mut module_file = PathBuf::from(&schemas_dir);
		module_file.push(format!("{}.rs", Self::parent_module_name()));
		std::fs::write(
			module_file,
			pretty_please(&schema_module_infos.to_module_string()),
		)
		.unwrap();
	}

	fn write_parent_module_folder(schemas_dir: &Path) {
		let mut module_dir = PathBuf::from(&schemas_dir);
		module_dir.push(Self::parent_module_name());
		std::fs::create_dir(&module_dir).unwrap();
	}
}

fn schemas_org_types_dir() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR"))
		.parent()
		.expect("should exist")
		.join("schema_org_types")
}

fn schemas_dir() -> PathBuf {
	schemas_org_types_dir().join("src").join("schemas")
}

fn write_schemas_org_types_manifest(features: FeatureMap) {
	const FEATURES_MARKER: &str = "# == GENERATED FEATURES ==";

	let manifest_path = schemas_org_types_dir().join("Cargo.toml");

	let mut manifest = std::fs::read_to_string(&manifest_path).expect("failed to read Cargo.toml");
	let marker_pos = manifest
		.rfind(FEATURES_MARKER)
		.expect("missing features marker comment");

	manifest.truncate(marker_pos + FEATURES_MARKER.len());
	manifest.push('\n');

	let mut features_flat = Arc::into_inner(features)
		.unwrap()
		.into_inner()
		.unwrap()
		.into_iter()
		.collect::<Vec<(_, _)>>();
	features_flat.sort_by_cached_key(|(feature, _)| feature.clone());

	for (feature, mut deps) in features_flat {
		deps.sort();

		write!(&mut manifest, "{feature} = [").unwrap();

		let total_deps = deps.len();
		for (index, dep) in deps.iter().enumerate() {
			let sep = if index + 1 == total_deps { "" } else { ", " };
			write!(&mut manifest, "\"{dep}\"{sep}").unwrap();
		}

		writeln!(&mut manifest, "]").unwrap();
	}

	std::fs::write(manifest_path, manifest).unwrap();
}

fn write_schemas(
	store: &Store,
	schemas_dir: &Path,
	schemas: &[SchemaQuerySolution],
	multi_progress: &MultiProgress,
) {
	Class::write_parent_module_folder(&schemas_dir);
	Property::write_parent_module_folder(&schemas_dir);
	Enumeration::write_parent_module_folder(&schemas_dir);
	DataType::write_parent_module_folder(&schemas_dir);

	let class_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let property_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let enumeration_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let data_types_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());

	let feature_set = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));

	let bar = multi_progress.add(ProgressBar::new(schemas.len() as u64));
	schemas.into_par_iter().for_each(|solution| {
		match NodeType::from_iri(store, &solution.iri) {
			NodeType::EnumerationVariant => {}
			NodeType::Property => {
				Property::handle_write(
					store,
					solution,
					property_schema_module_infos.lock().unwrap(),
					&schemas_dir,
					feature_set.clone(),
				);
			}
			NodeType::DataType => {
				DataType::handle_write(
					store,
					solution,
					data_types_schema_module_infos.lock().unwrap(),
					&schemas_dir,
					feature_set.clone(),
				);
			}
			NodeType::Enumeration => {
				Enumeration::handle_write(
					store,
					solution,
					enumeration_schema_module_infos.lock().unwrap(),
					&schemas_dir,
					feature_set.clone(),
				);
			}
			NodeType::Class => {
				Class::handle_write(
					store,
					solution,
					class_schema_module_infos.lock().unwrap(),
					&schemas_dir,
					feature_set.clone(),
				);
			}
		};
		bar.inc(1);
	});

	Class::write_parent_module(
		class_schema_module_infos
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&schemas_dir,
	);
	Property::write_parent_module(
		property_schema_module_infos
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&schemas_dir,
	);
	Enumeration::write_parent_module(
		enumeration_schema_module_infos
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&schemas_dir,
	);
	DataType::write_parent_module(
		data_types_schema_module_infos
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&schemas_dir,
	);

	write_schemas_org_types_manifest(feature_set);
}

pub fn write(store: &Store, multi_progress: &MultiProgress) {
	let schemas_dir = schemas_dir();
	std::fs::remove_dir_all(&schemas_dir).unwrap();
	std::fs::create_dir(&schemas_dir).unwrap();

	let mut schemas = store.get_schemas();
	schemas.sort_by_cached_key(|s| s.iri.clone());

	write_schemas(store, &schemas_dir, &schemas, multi_progress);
}
