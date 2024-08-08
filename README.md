
Use `cargo run --release -- --gizmos_immediate`, etc... to select specific rendering method. Press B to benchmark. 

Use `cargo run --release -- --benchmark` to automatically run all of the benchmarks.

For benchmarking, please lock GPU/VRAM clocks: [NVIDIA Instructions](https://developer.nvidia.com/blog/advanced-api-performance-setstablepowerstate/). And wait for rust-analyser, etc.. to cool down.

100k lines, 7950x, RTX3060, locked GPU clocks:
```
  1.82ms: bevy_lines_example_retained
  9.03ms: bevy_plane_3d_retained
  2.89ms: bevy_plane_3d_retained_combined
  3.03ms: gizmos_immediate
  4.33ms: gizmos_immediate_nan
  2.92ms: gizmos_immediate_continuous_polyline
 17.24ms: bevy_vector_shapes_retained
 18.15ms: bevy_vector_shapes_immediate
 89.76ms: bevy_polyline_retained
  2.78ms: bevy_polyline_retained_nan
  2.77ms: bevy_polyline_retained_continuous_polyline
```