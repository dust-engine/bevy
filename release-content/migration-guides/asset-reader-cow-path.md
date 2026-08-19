---
title: "`AssetReader` takes `CowArc<Path>`, and `LoadContext::read_asset` was added"
pull_requests: []
---

`AssetReader::read`, `AssetReader::read_meta` and `AssetReader::read_meta_bytes` (and their
`ErasedAssetReader` counterparts) now take `CowArc<'a, Path>` instead of `&'a Path`.

Previously the returned reader borrowed the path, which tied the reader's lifetime to the caller's
path. Passing an owned-or-borrowed `CowArc` lets a reader hold on to the path, which in turn lets
`LoadContext` hand out a reader that outlives the `LoadContext` borrow.

This enables the new `LoadContext::read_asset`, which returns the `Reader` for a dependency instead
of eagerly reading it all into memory the way `read_asset_bytes` does. `read_asset_bytes` is
unchanged and is now implemented on top of `read_asset`.

## Migration Steps

For custom `AssetReader` implementations, change the `path` parameter type:

```rust
// 0.18
async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
    // ...
}

// 0.19
async fn read<'a>(
    &'a self,
    path: CowArc<'a, Path>,
) -> Result<impl Reader + 'a, AssetReaderError> {
    // ...
}
```

Inside the body, `CowArc<Path>` derefs to `Path`, so most uses need no change. Where a `&Path` is
required explicitly, use `&path`.

For callers, `AssetPath::path_cow()` returns the `CowArc<'_, Path>` to pass in, in place of
`AssetPath::path()`.
