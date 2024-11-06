
Use `cargo run --release -- --gizmos_immediate`, etc... to select specific rendering method. Press B to benchmark. 

Use `cargo run --release -- --benchmark --auto_count` to automatically run all of the benchmarks with automatic line counts (more accurate).
Auto line counts starts at 50k and doubles the line count until the frame times is above 8ms. **Note: Using auto lines counts can result in flashing colors as meshes spawn/despawn.**

Use `cargo run --release -- --benchmark` to automatically run all of the benchmarks with fixed line counts.


For benchmarking, please lock GPU/VRAM clocks: [NVIDIA Instructions](https://developer.nvidia.com/blog/advanced-api-performance-setstablepowerstate/). And wait for rust-analyser, etc.. to cool down.

7950x, RTX3060, locked GPU clocks:
```
2056.8k lines/ms: bevy_lines_example_retained
  11.2k lines/ms: bevy_plane_3d_retained
 820.6k lines/ms: bevy_plane_3d_retained_combined
  41.9k lines/ms: gizmos_immediate
  26.2k lines/ms: gizmos_immediate_nan
  59.7k lines/ms: gizmos_immediate_continuous_polyline
   5.9k lines/ms: bevy_vector_shapes_retained
   5.7k lines/ms: bevy_vector_shapes_immediate
   1.2k lines/ms: bevy_polyline_retained
 403.9k lines/ms: bevy_polyline_retained_nan
 590.4k lines/ms: bevy_polyline_retained_continuous_polyline
```
