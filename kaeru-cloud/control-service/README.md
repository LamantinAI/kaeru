# Control service boundary

Control owns platform users, organizations, projects, membership, grants,
initiative placement and memory/control token administration. Identity and login
belong to Auth or an external IdP. Memory owns knowledge, recorded history and
query execution.

Control implementation and its service-authentication contract are a separate
stage. This directory intentionally contains no executable, placeholder
handlers or simulated authorization client. The current memory service's
instance bearer/account policy does not implement platform identity or grants.
