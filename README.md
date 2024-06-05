# DepthZ

A tool for analyzing git repos and correlating the depedencies between any related systems and libraries.

- Near-term goal: Export a textual graph of the systems and visualize it as a system map.
- Long-term goal: Correlate the graph with actual systems and do realtime analysis.

## Track Dependencies

Everything starts in a DEPTHZ file. See [Specification](./SPECIFICATION.md).

## Development

### Dependencies

- `git` - The binary calls out to a git subprocess to execute cloning/pulling

Note: Using git from the cli was a much simpler approach than trying to embed and use the `git2` library.

### Nix

This command will pull in any necessary deps if using nix:

```
nix develop
```

## Usage

Build an entry point DEPTHZ. This can reside locally, or put in its own repo.
To handle monorepo-like setups, or placing DEPTHZ elsewhere besides the root,
an optional `path` parameter can be set.

```
{
  "name": "DomainA",
  "type": "domain",
  "repos": [
    { "url": "git@host:repoA.git", "name": "repoA" },
    { "url": "git@host:repoB.git", "name": "repoB", "path": "/path/to/depthz/dir" }
  ]
}
```

Build any dependent DEPTHZ files and put them in their relevant repos.

```
{
  "name": "ServerA",
  "type": "server",  
  "elements": [
    { "name": "AppA",
      "type": "service"
    }
  ]
}
```

Then run the `depthz` command to generate a mermaid output.

```
# read locally
depthz -p DEPTHZ

# read out of a git repo with a nested DEPTHZ
depthz -g git@github.com:myrepo.git -n myrepo -p /my/DEPTHZ

# output to a file
depthz -p DEPTHZ -f output.mmd
```
Note that DEPTHZ files can be read independently to render their sub-set of the graph.

## Mermaid Output

```
flowchart TB
    DomainA --> Grafana
    DomainA --> Loki
    DomainA --> ServerA
    ServerA --> AppA
    AppA --> PosgresA
    PosgresA --> DatabaseA
    AppA --> ExternalA
    AppA --> InternalA
    AppA -->|1.0|LibraryA
```

## Mermaid Flow Chart

![mermaid example](docs/mermaid-example.png)
