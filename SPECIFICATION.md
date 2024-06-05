The default format for a DEPTHZ specification is JSON.

## DEPTHZ

Entry point to a working DepthZ system. Declares git repos to walk over, and aggregates
everything into a singular output.

### Entry point example

```
{
  "name": "DomainA",
  "type": "domain",
  "repos": [
    { "url": "git@host:repoA.git", "name": "repoA" },
    { "url": "git@host:repoB.git", "name": "repoB" },
    { "url": "git@host:repoC.git", "name": "repoC", "path": "/path/to/depthz/file" }
  ],
  "elements": [
    { "name": "Grafana", "type": "service" },
    { "name": "Loki", "type": "service" }
  ]
}
```

Stored inside each individual git repository are more DEPTHZ files. Declares any dependencies in its chain.
Gets consumed by a process reading over an entry DEPTHZ and integrated into the aggregate output.

These could also declare more sub-dependencies using `repos`.

### Example

```
{
  "name": "ServerA",
  "type": "server",  
  "elements": [
    { "name": "AppA",
      "type": "service",
      "elements": [
        { "name": "PosgresA",
          "type": "database",
          "elements": [{ "name": "DatabaseA", "type": "other" }]
        },
        { "name": "ExternalA", "type": "service" },
        { "name": "InternalA", "type": "service" },
        { "name": "LibraryA", "type": "library", "version": "1.0" }
      ]
    }
  ]
}
```

## Keywords

- name: Dependency identifier
- type: [domain, server, service, database, library, other]
- version (optional): Version identifier for dependency
- elements: A nested list of dependencies under an element of `type`
