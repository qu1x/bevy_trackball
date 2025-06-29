# Version 0.12.0 (2025-06-19)

  * Allow to set transmission ratio of movement to input per operation and input type.
  * Fix/Workaround panic on exit in examples.

# Version 0.11.0 (2025-04-25)

  * Bump `bevy` to `0.16.0`.
  * Bump `bevy_egui` to `0.34.0`.

# Version 0.11.0-rc.1 (2025-04-13)

  * Bump `bevy` to `0.16.0-rc`.
  * Bump `bevy_egui` to `0.34.0-rc`.

# Version 0.10.0 (2025-02-22)

  * Rust Edition 2024.
  * Bump MSRV.
  * Bump `bevy_egui` to `0.33.0`.
  * Bump `trackball` to `0.16.0` fixing `c11-orbit` feature.

# Version 0.9.0 (2024-11-30)

  * Bump MSRV to `1.82.0`.
  * Bump `bevy` to `0.15.0`.
  * Bump `bevy_egui` to `0.31.1`.
  * Bump `trackball` to `0.15.0`.

# Version 0.8.0 (2024-10-27)

  * Update `bevy_egui`.
  * Bump MSRV.
  * Disable wireframe mode in examples when unsupported.

# Version 0.7.0 (2024-07-06)

  * Update `bevy` to `0.14.0`.
  * Update `trackball` to `0.14.0`.
  * Improve examples, avoid CDN.

# Version 0.6.0 (2024-05-01)

  * Update `bevy_egui` to `0.27.0`.
  * Update `bevy` to `0.13.2`.
  * Document optional features.
  * Move webpage avoiding redirects and repository bloat.

# Version 0.5.0 (2024-03-19)

  * Update `bevy_egui` to `0.26.0`.
  * Update `bevy` to `0.13.1`.

# Version 0.4.1 (2024-03-06)

  * Add missing re-normalization as part of smoothing, thanks to [mo8it](https://github.com/mo8it).
    This prevents panics when dependants enable the `glam_assert`/`debug_glam_assert` feature.

# Version 0.4.0 (2024-02-21)

  * Update Bevy to `0.13`.
  * Translate blend ratio of `0.25` into half-life of `40.0` milliseconds.
  * Support creating `Delta::Track` events to follow a target.

# Version 0.3.0 (2023-12-05)

  * Fix mouse and especially touch input by clearing event readers when viewport
    is given back. This issue got noticeable with Bevy `0.12.1`, see [#10877].
  * Refactor viewport stealing allowing to specify number of frames.
  * Let `m` key toggle between `map_esdf` and `map_wasd` for slide operations.

[#10877]: https://github.com/bevyengine/bevy/issues/10877

# Version 0.2.0 (2023-11-07)

  * Update Bevy to `0.12`.

# Version 0.1.1 (2023-08-20)

  * Fix default of `TrackballInput::first_key` matching documentation.
  * Host documentation next to examples. This fixes outdated intra-doc links to `nalgebra` `0.25.0`
    and broken intra-doc links to `bevy`.

# Version 0.1.0 (2023-08-19)

  * Initial release.
