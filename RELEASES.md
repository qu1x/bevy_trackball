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
