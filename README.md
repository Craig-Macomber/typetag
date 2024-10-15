# Modified typetag

Includes rebased changes from https://github.com/dtolnay/typetag/pull/36 to add dynamic registration API to work around platform support limitations of `inventory` which does not currently report any registrations when used on `wasm`.

These changes do not reflect the intended API / design as expressed by the creator of TypeTag, and as its a fork this version will most likely get much poorer support and maintenance.
For the true and much more elegant (but on wasm not so useful at time of writing) typetag experience, use the original version from https://github.com/dtolnay/typetag and not this hacky fork.
