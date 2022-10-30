crates := "cataas cataacli"

@_default:
    just --list

# Publish all crates.
@publish-all:
    for crate in {{ crates }}; do \
        just publish "$crate"; \
    done

# Publish a crate.
@publish CRATE:
    cargo publish -p {{ CRATE }}
