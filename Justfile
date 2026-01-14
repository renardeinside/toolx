
# Release with automatic version sync
release *tag:
    #!/usr/bin/env bash
    # Update Cargo.toml with the tag version (remove 'v' prefix)
    VERSION=$(echo "{{tag}}" | sed 's/^v//')
    sed -i '' "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
    git add Cargo.toml
    git commit -m "bump version to {{tag}}"
    git tag {{tag}}
    git push origin main {{tag}}

# Build wheel with current git tag version
build:
    uv build --wheel