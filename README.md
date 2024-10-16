# Modified typetag

Includes rebased changes from https://github.com/dtolnay/typetag/pull/36 to add dynamic registration API to work around platform support limitations of `inventory` which does not currently report any registrations when used on WASM.

These changes were rebased by me (Craig), and were done with a poor understanding of most of the things involved (`typetag`, the original path from from AaronFriel, procedural macros and their related libraries).
Thus this branch isn't something you should trust, and probably gets several things wrong.
It works for my use-case and I have made it public mostly because thats the easiest way for me to consume this patch.
If you want to use this, please confirm that you cannot actually use upstream typetag first, then review the changes this branch makes before using.

These changes do not reflect the intended API / design as expressed by the creator of TypeTag, and as its a fork by this version will most likely get much poorer support and maintenance.
For the true and much more elegant (but on wasm not so useful at time of writing) typetag experience, use the original version from https://github.com/dtolnay/typetag and not this hacky fork.

This modified version of typetag adds a "runtime" feature, that when enabled requires you to manually register each implementation before it's used.
For example at the start of main, you can do something like:

```rust
<PageLoad as WebEvent>::register();
<Click as WebEvent>::register();
```

When running in WASM, this actually works since it avoid the dependency on `inventory`.
