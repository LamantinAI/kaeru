# Platform contracts

Typed platform authorization models shared by the tokenizer and future Control
and Memory adapters. This crate has no knowledge-graph, SQL, HTTP or cache
dependency. Exchange types live in `src/models/`.

Action IDs and known permission bits are explicit. Product credentials use
`SecretToken`, whose debug output is redacted. The serializer still emits the
credential for an authorized wire exchange; do not log or persist request bodies.

See [the tokenizer service](../tokenizer-service/README.md) for
API, state ownership and tests.
