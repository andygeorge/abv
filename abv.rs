use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use serde_yaml;

#[derive(Deserialize)]
struct Config {
    ansible: AnsibleConfig,
}

#[derive(Deserialize)]
struct AnsibleConfig {
    config_file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault_name = env::args().nth(1).expect("vault name argument is required");
    let vault_file_path = env::args().nth(2).expect("vault file path argument is required");

    let abv_config_path = Path::new(&env::var("HOME")?).join(".abv.cfg");
    let abv_config_content = std::fs::read_to_string(abv_config_path)?;
    let abv_config: HashMap<String, HashMap<String, String>> = serde_yaml::from_str(&abv_config_content)?;

    let ansible_config_path = Path::new(&env::var("HOME")?)
        .join(abv_config["ansible"]["config_file_path"].replace("~", &env::var("HOME")?));
    let ansible_config_content = std::fs::read_to_string(ansible_config_path)?;
    let ansible_config: Config = serde_yaml::from_str(&ansible_config_content)?;

    let vault_identity_list: Vec<&str> = ansib_config["defaults"]["vault_identity_list"]
        .split(", ")
        .collect();
    let mut vault_identity_hash = HashMap::new();

    for identity in vault_identity_list {
        let parts: Vec<&str> = identity.split("@").collect();
        if parts.len() == 2 {
            vault_identity_hash.insert(parts[1], parts[0]);
        }
    }

    let vault_file_contents = std::fs::read_to_string(vault_file_path)?;
    let vault_string = vault_file_contents
        .lines()
        .find(|line| line.contains(vault_name))
        .expect("Vault name not found in file");

    let vault_string_descriptor = vault_string.lines().next().unwrap().trim_end();
    let vault_id = vault_string_descriptor.split(';').last().unwrap();

    let vault_password_file = Path::new(&vault_identity_hash[vault_id]).join(
        env::var("HOME").expect("Home directory not found in environment"),
    );

    let vault_string_decrypted = std::process::Command::new("ansible-vault")
        .args(&["decrypt", "--vault-password-file", vault_password_file.to_str().unwrap()])
        .input(vault_string)
        .output()?;

    if !vault_string_decrypted.status.success() {
        eprintln!("Error decrypting vault: {:?}", vault_string_decrypted.stderr);
        std::process::exit(1);
    }

    println!("{}", String::from_utf8_lossy(&vault_string_decrypted.stdout));

    Ok(())
}
