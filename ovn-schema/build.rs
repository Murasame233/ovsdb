use std::{env::current_dir, fs::remove_dir_all};

fn main() {
    remove_dir_all("src/vnetwork").unwrap();
    ovsdb_build::configure()
        .compile(
            format!(
                "{}/ovn.ovsschema",
                current_dir().unwrap().to_str().unwrap()
            ),
            "src/vnetwork".into(),
        )
        .unwrap();
}
