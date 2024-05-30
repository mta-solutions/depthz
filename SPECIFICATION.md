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
    { "url": "git@host:repoC.git", "name": "repoC" }
  ],
  "elements": [
    { "name": "Grafana", "type": "service" },
    { "name": "Loki", "type": "service" }
  ]
}
```

Stored inside each individual git repository are more DEPTHZ files. Declares any dependencies in its chain.
Gets consumed by a process reading over an entry DEPTHZ and integrated into the aggregate output.

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

- domain: A class of servers and services which encompass an entire service area
  - name (optional): Domain identifier
  - repos: List of git repositories to be walked over and consumed
- deps: A list of dependencies under a domain, like a server or service
  - name: Dependency identifier
  - type: [domain, server, service, database, library, other]
  - version (optional): Version identifier for dependency
