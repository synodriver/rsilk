<h1 align="center"><i>✨ rsilk ✨ </i></h1>

<h3 align="center">The python binding for <a href="https://github.com/lz1998/silk-rs">silk-rs</a> </h3>


[![pypi](https://img.shields.io/pypi/v/rsilk.svg)](https://pypi.org/project/rsilk/)
![python](https://img.shields.io/pypi/pyversions/rsilk)
![implementation](https://img.shields.io/pypi/implementation/rsilk)
![wheel](https://img.shields.io/pypi/wheel/rsilk)
![license](https://img.shields.io/github/license/synodriver/rsilk.svg)
![action](https://img.shields.io/github/workflow/status/synodriver/rsilk/build%20wheel)




## Usage

- encode

```python
import rsilk

with open("tests/input.pcm", "rb") as f:
    data = f.read()

output = rsilk.encode(data, 24000, 24000)
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

- def decode(src: bytes, sample_rate: int, frame_size: int = 0, frames_per_packet: int = 1, more_internal_decoder_frames: bool = False, in_band_fec_offset: int = 0, loss: bool = False) -> bytes


- def encode(input: bytes, sample_rate: int, bit_rate: int, max_internal_sample_rate: int = 24000, packet_loss_percentage: int = 0, complexity: int = 2, use_inband_fec: bool = False, use_dtx: bool = False, tencent: bool = True) -> bytes