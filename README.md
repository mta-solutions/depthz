# DepthZ

A tool for analyzing git repos and correlating the depedencies between any related systems and libraries.

- Near-term goal: Export a textual graph of the systems and visualize it as a system map.
- Long-term goal: Correlate the graph with actual systems and do realtime analysis.

## Track Dependencies

Everything starts in a DEPTHZ file. See [Specification](./SPECIFICATION.md).

- Domains (aka service areas)
- App, system, server relationships
- Library relationships

## Example

Creating a system dependency chart would be useful.

```
-- App Level
AppA -> PostgresA -> DatabaseA
     -> 3rd party external internet service
     -> 3rd party internal vendor system
     -> LibraryA
AppB -> PostgresA -> DatabaseB
     -> CacheA
     -> LibraryA

-- Server Level
ServerA -> NomadA -> AppA
                  -> AppB
ServerB -> NomadB -> AppA
                  -> AppB

-- Domain Level
DomainA -> ServerA
        -> ServerB
```

Note that 'PostgresA' is a shared service, so if it goes down both 'AppA' and 'AppB' are considered down.

With the server level tracking, services could be distributed such as with the `NomadX` example.
A nomad instance or app going down in this scenario doesn't always mean the entire service is down, just degraded.

Tracking shared libraries would help build a dependency list for developers.
