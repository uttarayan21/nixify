## Nixify

A simple cli tool to convert `toml`/`yaml`/`json` data into `nix` expressions.

### Example

```bash
cat example.yaml | nixify 
```

you can also specify the format if you don't want it guessing
or a toml file

```bash
cat Cargo.toml | nixify -f toml
```

you can also run directly without installing like so

```bash
cat Cargo.toml | nix run github:uttarayan21/nixify
```

Which turns into

```toml
{
  dependencies = {
    clap = {
      features = [
        "derive"
      ];
      version = "4.5";
    };
    clap_complete = "4.5";
    error-stack = "0.6";
    ser_nix = "0.2.2";
    serde = {
      features = [
        "derive"
      ];
      version = "1.0.228";
    };
    serde_json = "1.0.149";
    serde_yml = "0.0.12";
    thiserror = "2.0";
    toml = "1.0.3";
  };
  package = {
    edition = "2024";
    license = "MIT";
    name = "nixify";
    version = "0.1.0";
  };
}
```
