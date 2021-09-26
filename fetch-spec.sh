#!/bin/sh -ex
# Fetch specs for onfido-openapi-spec $version
#
# Spec: https://github.com/onfido/onfido-openapi-spec

: "${version:="v2.1.0-beta"}"

if test -d spec; then rm -r spec; fi
mkdir spec

curl \
    --fail \
    --location \
    "https://github.com/onfido/onfido-openapi-spec/archive/refs/tags/${version}.tar.gz" \
    |
tar -xz \
    --directory 'spec' \
    --strip-components=1

# XXX: Hack some relative paths to make openapi-generator work.
sed \
    --in-place \
    --expression 's#ref: #ref: schemas/reports/#' \
    --expression 's#ref: schemas/reports/report_document[.]yaml#ref: report_document.yaml#' \
    'spec/schemas/reports/report.yaml'
