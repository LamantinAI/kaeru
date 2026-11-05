# Platform services

This directory is not a Cargo package.

- [`memory-service`](memory-service/) serves MCP, HTTP, the instance UI and the
  legacy sharing adapter on one Store. Its default storage is TiKV.
- [`control-service`](control-service/) reserves the ownership boundary for
  platform users, organizations, projects, permissions and token administration.

Core owns graph semantics. The shared `kaeru-server` library contains reusable
transport implementation; it is not a separately deployed service.
