## Python Wrapper for Rust regex crate  

### Install:  
    pip install py-rust-regex

### Usage:  

```python
import py_rust_regex as pr
reg = pr.new_regex('[0-9]+')
for match in reg.find_all('abc123def456'):
    print(match.range.start, match.range.end, match.len)
```

### Github:  
https://github.com/dttvn0010/py-rust-regex
