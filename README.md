
Use `cargo run --release -- --gizmos_immidate`, etc... to select rendering method. Press B to benchmark. 

For benchmarking, please lock GPU/VRAM clocks: [NVIDIA Instructions](https://developer.nvidia.com/blog/advanced-api-performance-setstablepowerstate/). And wait for rust-analyser, etc.. to cool down.

100k lines, RTX3060, locked GPU clocks:
```
 1.81ms: bevy_lines_example_retained
 3.03ms: gizmos_immidate
 5.25ms: bevy_polyline_retained
 9.51ms: bevy_plane_3d_retained
17.42ms: bevy_vector_shapes_retained
19.80ms: bevy_vector_shapes_immidate
```