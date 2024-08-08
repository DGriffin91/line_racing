
Use `cargo run --release -- --gizmos_immediate`, etc... to select specific rendering method. Press B to benchmark. 

Use `cargo run --release -- --benchmark` to automatically run all of the benchmarks.

For benchmarking, please lock GPU/VRAM clocks: [NVIDIA Instructions](https://developer.nvidia.com/blog/advanced-api-performance-setstablepowerstate/). And wait for rust-analyser, etc.. to cool down.

150k lines, 7950x, RTX3060, locked GPU clocks:
```
  0.55ms: bevy_lines_example_retained
 13.44ms: bevy_plane_3d_retained
  0.59ms: bevy_plane_3d_retained_combined
  3.70ms: gizmos_immediate
  5.28ms: gizmos_immediate_nan
  2.71ms: gizmos_immediate_continuous_polyline
 25.75ms: bevy_vector_shapes_retained
 26.86ms: bevy_vector_shapes_immediate
140.38ms: bevy_polyline_retained
  0.55ms: bevy_polyline_retained_nan
  0.54ms: bevy_polyline_retained_continuous_polyline
```