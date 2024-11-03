use extism_convert::Json;
use extism_pdk::*;
use serde::Deserialize;
use std::fs;
use toml_edit::{value, DocumentMut};

const CARGO_TOML_PATH: &str = "Cargo.toml";

#[derive(Debug, Deserialize)]
struct SetVersionRequest {
    version: String,
}

#[plugin_fn]
pub fn get_version(_: String) -> FnResult<String> {
    let version = get_toml_value(CARGO_TOML_PATH);
    match version {
        Ok(contents) => Ok(contents),
        Err(e) => Err(WithReturnCode::new(e, 1)),
    }
}

/// Get the version of the package from the `Cargo.toml` file.
fn get_toml_value(toml_path: &str) -> Result<String, Error> {
    let file_contents = std::fs::read_to_string(toml_path)?;
    let document = file_contents.parse::<DocumentMut>()?;
    let version = document["package"]["version"].as_str().unwrap().to_string();
    Ok(version)
}

#[plugin_fn]
pub fn set_version(input: Json<SetVersionRequest>) -> FnResult<()> {
    update_package_section(CARGO_TOML_PATH, &input.0.version)?;

    Ok(())
}

/// Update the version of the package in the `Cargo.toml` file to the given version.
fn update_package_section(file_path: &str, new_version: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(file_path)?;

    let mut document = contents.parse::<DocumentMut>()?;

    if let Some(package) = document.get_mut("package") {
        if let Some(version) = package.get_mut("version") {
            *version = value(new_version.to_string());
        }
    }

    fs::write(file_path, document.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tempfile::tempdir;

    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[rstest]
    #[case(
        "[package]\nname = \"extism-convert\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0.213\"\n",
        "0.1.0"
    )]
    fn test_get_version(#[case] input: &str, #[case] expected: &str) {
        let dir = tempdir().unwrap();

        let temp_file = dir.path().join(CARGO_TOML_PATH);
        let mut file = File::create(&temp_file).unwrap();

        write!(file, "{}", input).unwrap();

        let version = get_toml_value(temp_file.to_str().unwrap()).unwrap();

        similar_asserts::assert_eq!(version, expected);
    }

    #[rstest]
    #[case(
        "[package]\nname = \"extism-convert\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0.213\"\n",
        "1.0.0",
        "[package]\nname = \"extism-convert\"\nversion = \"1.0.0\"\n\n[dependencies]\nserde = \"1.0.213\"\n"
    )]
    fn test_update_package_section(
        #[case] input: &str,
        #[case] new_version: &str,
        #[case] expected: &str,
    ) {
        let dir = tempdir().unwrap();

        let temp_file = dir.path().join(CARGO_TOML_PATH);
        let mut file = File::create(&temp_file).unwrap();

        write!(file, "{}", input).unwrap();

        update_package_section(temp_file.to_str().unwrap(), new_version).unwrap();

        let updated_contents = fs::read_to_string(temp_file).unwrap();

        similar_asserts::assert_eq!(updated_contents, expected);
    }
}
