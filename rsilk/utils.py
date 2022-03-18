from io import BytesIO
from typing import Tuple

from rsilk._silk import encode

try:
    import av
except ImportError:
    av = None


if av is not None:
    def to_pcm_bytes(in_path: BytesIO) -> Tuple[bytes, int]:
        """任意媒体文件转 pcm bytes"""
        out_path = BytesIO()
        with av.open(in_path) as in_container:
            in_stream = in_container.streams.audio[0]
            sample_rate = in_stream.codec_context.sample_rate
            with av.open(out_path, 'w', 's16le') as out_container:
                out_stream = out_container.add_stream(
                    'pcm_s16le',
                    rate=sample_rate,
                    layout='mono'
                )
                try:
                    for frame in in_container.decode(in_stream):
                        frame.pts = None
                        for packet in out_stream.encode(frame):
                            out_container.mux(packet)
                except:
                    pass
        return out_path.getvalue(), sample_rate


    def to_pcm_file(in_path: BytesIO, out_path: BytesIO) -> int:
        """任意媒体文件转 pcm file"""
        with av.open(in_path) as in_container:
            in_stream = in_container.streams.audio[0]
            sample_rate = in_stream.codec_context.sample_rate
            with av.open(out_path, 'w', 's16le') as out_container:
                out_stream = out_container.add_stream(
                    'pcm_s16le',
                    rate=sample_rate,
                    layout='mono'
                )
                try:
                    for frame in in_container.decode(in_stream):
                        frame.pts = None
                        for packet in out_stream.encode(frame):
                            out_container.mux(packet)
                except:
                    pass
        return sample_rate


    def convert_to_silk(media_path: BytesIO) -> bytes:
        """任意媒体文件转 silk, 返回silk路径"""
        pcm_bytes, sample_rate = to_pcm_bytes(media_path)
        return encode(pcm_bytes, sample_rate, sample_rate, True)
