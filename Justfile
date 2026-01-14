
# Release with automatic version sync
release *tag:
    #!/usr/bin/env bash
    # Update Cargo.toml with the tag version (remove 'v' prefix)
    VERSION=$(echo "{{tag}}" | sed 's/^v//')
    cargo set-version $VERSION
    git commit -am "Release {{tag}}"
    git tag {{tag}}
    git push origin main --tags