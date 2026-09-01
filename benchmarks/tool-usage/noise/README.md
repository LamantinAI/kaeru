# Noise libraries

Design rule 2: *a fixture must reproduce scale, not only structure*. A four-node
fixture puts the target on the surface; the field failures happen in working sets
where the right node does not surface first.

Each library is a bank of nodes sharing vocabulary with a domain. A case includes
one and says how the target must sit inside it:

```yaml
noise:
  library: ../../noise/tooling-noise.yaml
  count: 40
  target_rank_at_least: 4   # target must NOT be in the first N hits
  target_not_newest: true   # recency must not hand it over for free
```

Noise nodes are deliberately dull and plausible: they mention the same tools and
topics, and none of them answers the case's question. If a noise node can be
mistaken for the answer, it belongs in the fixture as a distractor, not here.
