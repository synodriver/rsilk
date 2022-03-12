# rsilk

the python binding for [silk-rs](https://github.com/lz1998/silk-rs)

## Usage

- encode

```python
import rsilk

with open("input.pcm", "rb") as f:
    data = f.read()

output = rsilk.encode(data, 24000, 24000, True)
with open("output.silk", "wb") as f:
    f.write(output)
```

- decode

```python
import rsilk

with open("input.silk", "rb") as f:
    data = f.read()

output = rsilk.decode(data, 24000)
with open("output.pcm", "wb") as f:
    f.write(output)
```

### Exceptions

- SilkError

### Public functions

- def decode(src: bytes, sample_rate: int) -> bytes

- def encode(input: bytes, sample_rate: int, bit_rate: int, tencent: bool) -> bytes