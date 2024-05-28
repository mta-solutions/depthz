The default format for a DEPTHZ specification is JSON.

## DEPTHZ

Entry point to a working DepthZ system. Declares git repos to walk over, and aggregates
everything into a singular output.

### Entry point example

```
{
  "domain": {
    "name": "DomainA",
    "repos": [
      "git@host:repoA.git",
      "git@host:repoB.git",
      "git@host:repoC.git"
    ],
    "services": ["Grafana", "Loki"]
  }
}
```

Stored inside each individual git repository are more DEPTHZ files. Declares any dependencies in its chain.
Gets consumed by a process reading over an entry DEPTHZ and integrated into the aggregate output.

### Example

```
{
  "servers": {
    "name": "ServerA",  
    "services": [
      { "name": "AppA",
        "dependencies": [
          { "services": [
            { "name": "PosgresA",
              "dependencies": "DatabaseA"
            }
          },
          "ExternalA",
          "InternalA"
        ],
        "libraries": [{"name": "LibraryA", "version": "1.0"}]
      }
    ]
  }
}
```

## Keywords

- domain: A class of servers and services which encompass an entire service area
  - name (optional): Domain identifier
  - repos: List of git repositories to be walked over and consumed
- servers: A node which represents a bare-metal or VM instance
  - name: Server identifier
- services: A service which runs, whether custom or off the shelf
  - name: Service identifier
  - dependencies: Additional services which this service is dependent on
  - libraries: A custom or off the shelf code library
    - name: Library name
    - version (optional): The version of the library

Note that `dependencies` is a unique field which can recurse.
