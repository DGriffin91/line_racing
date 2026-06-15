
Use `cargo run --release -- --gizmos_immediate`, etc... to select specific rendering method. Press B to benchmark. 

Use `cargo run --release -- --benchmark --auto_count` to automatically run all of the benchmarks with automatic line counts (more accurate).
Auto line counts starts at 50k and doubles the line count until the frame times is above 8ms. **Note: Using auto lines counts can result in flashing colors as meshes spawn/despawn.**

Use `cargo run --release -- --benchmark` to automatically run all of the benchmarks with fixed line counts.


For benchmarking, please lock GPU/VRAM clocks: [NVIDIA Instructions](https://developer.nvidia.com/blog/advanced-api-performance-setstablepowerstate/). And wait for rust-analyser, etc.. to cool down.

Bevy 0.18, 7950x, RTX4070ti, locked GPU clocks:
```
5946.5k lines/ms: bevy_lines_example_retained
   7.9k lines/ms: bevy_plane_3d_retained
2583.6k lines/ms: bevy_plane_3d_retained_combined
  41.2k lines/ms: gizmos_immediate
  22.0k lines/ms: gizmos_immediate_nan
  54.7k lines/ms: gizmos_immediate_continuous_polyline
   0.5k lines/ms: gizmos_retained
1599.6k lines/ms: gizmos_retained_combined
1598.6k lines/ms: gizmos_retained_continuous_polyline
   4.8k lines/ms: bevy_vector_shapes_retained
   3.8k lines/ms: bevy_vector_shapes_immediate
   0.8k lines/ms: bevy_polyline_retained
 528.3k lines/ms: bevy_polyline_retained_nan
1603.7k lines/ms: bevy_polyline_retained_continuous_polyline
```

Bevy 0.14, 7950x, RTX4070ti, locked GPU clocks:
```
5932.4k lines/ms: bevy_lines_example_retained
  11.6k lines/ms: bevy_plane_3d_retained
2625.9k lines/ms: bevy_plane_3d_retained_combined
  42.3k lines/ms: gizmos_immediate
  22.3k lines/ms: gizmos_immediate_nan
  56.1k lines/ms: gizmos_immediate_continuous_polyline
   5.2k lines/ms: bevy_vector_shapes_retained
   4.8k lines/ms: bevy_vector_shapes_immediate
   1.0k lines/ms: bevy_polyline_retained
 529.1k lines/ms: bevy_polyline_retained_nan
1607.2k lines/ms: bevy_polyline_retained_continuous_polyline
```
