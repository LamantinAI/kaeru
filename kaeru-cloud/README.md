# Platform services

This directory is not a Cargo package.

- [`contracts`](contracts/) contains shared platform DTOs, roles, actions and
  scope models, independent of graph storage and transports.
- [`memory-service`](memory-service/) serves MCP, HTTP, the instance UI and the
  legacy sharing adapter on one Store. Its default storage is TiKV.
- [`control-service`](control-service/) reserves the ownership boundary for
  platform users, organizations, projects and business permission assignments.
- [`tokenizer-service`](tokenizer-service/) owns credentials and effective
  authorization in PostgreSQL with optional Valkey caching. It is independently
  executable; Control/Memory integration is not yet wired.

Core owns graph semantics. The shared `kaeru-runtime` library contains reusable
transport implementation; it is not a separately deployed service.
