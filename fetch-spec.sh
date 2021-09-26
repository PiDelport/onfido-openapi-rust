#!/bin/sh -ex
# Fetch specs for onfido-openapi-spec $version
#
# Spec: https://github.com/onfido/onfido-openapi-spec

: "${version:="v1.0.0"}"

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
