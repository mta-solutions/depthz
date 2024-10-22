# DepthZ

A tool for analyzing git repos and correlating the depedencies between any related systems and libraries.

- Near-term goal: Export a textual graph of the systems and visualize it as a system map.
- Long-term goal: Correlate the graph with actual systems and do realtime analysis.

## Dependencies

- `git` - The binary calls out to a git subprocess to execute cloning/pulling

## Development

Note: Using git from the cli was a much simpler approach than trying to embed and use the `git2` library.

### Nix

This command will pull in any necessary deps if using nix:

```
nix develop
```

Running `depthz` commands using Rust's `cargo` build tool:

```
cargo run -- -p test/repo/b -d DEPTHZ.toml
```

# Track Dependencies

Everything starts in a DEPTHZ file.

## Format

DEPTHZ files can be declared as JSON, YAML, or TOML.

From the CLI, or in the `repo` sections, the `DEPTHZ` filename can be overridden.

## Usage

Build an entry point DEPTHZ. This can reside locally, or put in its own repo.
To handle monorepo-like setups, or placing DEPTHZ elsewhere besides the root,
an optional `path` parameter can be set.

The definition in each repo can independently be run against. If multiple repos
need to be linked together, create a `repos` section which points to each
relevant sub-repo and place this DEPTHZ inside the higher level domain repo.
From there it will clone and subsume the corresponding DEPTHZ files to build
out the full picture of the domain.

Optionally tag elements. If a filter is applied and an element doesn't match,
both itself and all child elemnents get filtered out. This is potentially useful
for determining loosely couple systems without a full overview, narrowing down
the graph scope to a tagged department in the company, etc.

**YAML**

```yaml
name: DomainA
type: domain
tags:
  - foo
  - bar
repos:
  - url: git@github.com:mta-solutions/depthz.git
    name: depthz
    path: /test/repo/a
    depthz: DEPTHZ
  - url: git@github.com:mta-solutions/depthz.git
    name: depthz
    path: /test/repo/b
    depthz: DEPTHZ.toml
```

**JSON**

```json
{
  "name": "DomainA",
  "type": "domain",
  "tags": [ "foo", "bar" ],
  "repos": [
    { "url": "git@github.com:mta-solutions/depthz.git",
      "name": "depthz",
      "path": "/test/repo/a",
      "depthz": "DEPTHZ"
	  },
    { "url": "git@github.com:mta-solutions/depthz.git",
      "name": "depthz",
      "path": "/test/repo/b",
      "depthz": "DEPTHZ.toml"
	  }
  ]
}
```

## Example Hierarchy

Build the top level DEPTHZ file.

`git@github.com/co/infra`:

```yaml
name: DomainA
type: domain
repos:
  - url: git@github.com:co/proj_a.git
    name: proj_a
    depthz: DEPTHZ
  - url: git@github.com:co/proj_b.git
    name: proj_b
    depthz: DEPTHZ
```

Build any dependent DEPTHZ files and put them in their relevant repos.

`git@github.com/co/proj_a`:


```yaml
name: ServerA
type: server
elements:
  - name: AppA
    type: service
```

`git@github.com/co/proj_b`:

```yaml
name: ServerB
type: server
elements:
  - name: AppB
    type: service
```

Then run the `depthz` command to generate a mermaid output.

```bash
# read locally
depthz -p DEPTHZ

# read out of a git repo with a nested DEPTHZ
depthz -g git@github.com:myrepo.git -n myrepo -p /my/DEPTHZ

# output to a file
depthz -p DEPTHZ -f output.mmd

# override the default DEPTHZ name
depthz -d DEPTHZ.toml

# filter elements by tags
depthz -t "foo,bar"
```

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
