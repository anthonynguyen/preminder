#[derive(Debug,Deserialize)]
pub struct Repository {
	pub name: String,
	pub full_name: String,
	pub description: Option<String>
}
