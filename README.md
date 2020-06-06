### Proxify
<i>A pretty shitty proxy parser for Rust</i>

### How to use?

Put proxify in your dependency section:

```toml
proxify = "0.1"
```

Use it:

```rust
let proxy = proxify::get_proxy().await;
println!("{:?}", proxy);
```
