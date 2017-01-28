#[derive(Serialize, Deserialize, Debug)]
pub struct HostResult {
    address: String,
    ports: HashMap<String, String>
}
