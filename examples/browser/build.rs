pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("Build icon module");
    Ok(())
}
