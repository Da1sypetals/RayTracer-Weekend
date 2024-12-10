# Run

## Binary
```
cargo run --bin iter
```

## Python Bindings
- First make sure `maturin` is installed.

```
cd crates/py-raytrace
maturin dev -r
cd ../..
python render.py
```

# Results

![](/assets/nice.png)
![](/assets/ballwall.png)
![](/assets/litup.png)