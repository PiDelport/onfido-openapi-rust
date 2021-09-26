#!/bin/sh -e
# Run the generator for the given OpenAPI spec files.
#
# Docs: https://openapi-generator.tech/docs/generators/rust

: "${version:="0.1.0"}"

# Post-process script: Trim the default Markdown documentation links from the generated README.
sed_readme_trim_docs="
/^## Documentation for API Endpoints$/,/^To get access to the crate's generated documentation, use:$/ c \
## Documentation\n\
\n\
To get access to the crate's generated documentation, use:
"

spec_file='spec/openapi.yaml'
name='onfido-openapi'

# Make sure the output directory is clean
if test -d "${name}"; then rm -r "${name}"; fi

./openapi-generator-cli generate \
    --generator-name rust \
    --reserved-words-mappings 'type=type_' \
    --input-spec "$spec_file" \
    --ignore-file-override '.openapi-generator-ignore' \
    --library 'reqwest' \
    --additional-properties "packageVersion=${version}" \
    --package-name "${name}" \
    --output "${name}"

# Post-process:
sed --in-place "$sed_readme_trim_docs" "${name}/README.md"

cargo fmt
