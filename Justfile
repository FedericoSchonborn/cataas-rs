crates := "cataas cataacli"

@_default:
    just --list

# Publish all crates.
@publish-all:
    just publish {{ crates }}

# Publish one or more crates.
@publish +CRATES:
    for crate in {{ CRATES }}; do \
        cargo publish -p "$crate"; \
    done
