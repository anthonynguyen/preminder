# `preminder`

preminder is a Rust program that creates reminders about Github pull requests.

## Configuration

Configuration is done via JSON/YAML/TOML. A sample YAML configuration file could
look like this:

```yaml
github:
  token: secret!
  host: github.organization.org
  subject: user

period: 6h

outputs:
  - type: stdout
    config: {}
```

## License

preminder is licensed under the MIT license. Please see the `LICENSE` file for
more details.
