# Onfido OpenAPI clients for Rust

Rust client libraries generated from Onfido's OpenAPI specs at [onfido-openapi-spec], using [openapi-generator].

## Usage

Reference the crates for the APIs you need from `Cargo.toml` using `git`:

```toml
[dependencies]
onfido-openapi = { git = "https://github.com/PiDelport/onfido-openapi-rust" }
```

Optionally, specify `branch`, or `tag` for a specific release.

## Version policy

The crate versions follow the upstream [onfido-openapi-spec] versions, but with a major version of `0` (instead of `1`).

## Spec updates

To update this repository for new upstream spec releases:

1. Fetch the latest spec:

   ```shell
   ./fetch-spec.sh
   ```

2. Regenerate the client crate:

   ```shell
   ./generate.sh
   ```

[onfido-openapi-spec]: https://github.com/onfido/onfido-openapi-spec
[openapi-generator]: https://github.com/OpenAPITools/openapi-generator
