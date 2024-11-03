# Rust Cargo Plugin

This is a plugin for [Universal Changesets](https://github.com/universal-changesets/core) that allows you to get/set the version of your Rust project via the `Cargo.toml` file.

## Usage

To use this plugin, you need to reference it within the `.changeset/config.json` file:

```json
{
  "$schema": "https://raw.githubusercontent.com/universal-changesets/core/main/changeset-config.schema.json",
  "plugin": {
    "sha256": "e63c184c019d2198b497ceeaefeb59587da138ca7f78edc34e21332a7cc4b18c",
    "url": "gh:universal-changesets/rust-cargo-plugin@1.0.0"
  }
}
```

Once you've done that, you can use the `changeset get` and `changeset version` functions in your project. Refer to the [documentation](https://github.com/universal-changesets/core/blob/main/docs/plugins.md) for more information.
