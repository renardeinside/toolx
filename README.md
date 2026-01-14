Repository to test the behaviour of GH pages acting as a package registry.


## How to release

1. Run `just release vX.Y.Z` to create a new release and tag it


## Checking the release

Go to [https://renardeinside.github.io/toolx/simple/toolx/](https://renardeinside.github.io/toolx/simple/toolx/) to see the list of releases.


## Using the release via `uvx`

```bash
uvx --verbose --index https://renardeinside.github.io/toolx/simple toolx --version
```