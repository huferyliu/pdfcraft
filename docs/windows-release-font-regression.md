# Windows release-font regression supplement for #426

This supplement adds a Windows test and reproduction instructions to the existing Chinese
runtime-font fix. Product changes remain authored by `sll1679339710-arch` in #426; this
supplement does not reimplement them or claim macOS/Linux validation.

## Release input and execution

The release workflow at `4f11e0f9de3d51b9f2a7c657dcda27ea25b44e71` pins craft-fonts to
`abb83316d96aa59c1cf64784289e378fe9fa5695`. Permitted Japanese fonts cover some shared Han
characters while leaving Simplified-only glyphs missing. A build without craft-fonts cannot
exercise that font-order condition. The fixture requires no embedded Hans face and first
asserts that embedded-only definitions reproduce missing Chinese filename glyphs.

Use the policy-compliant release-pinned input and an already installed Windows Chinese face
such as Microsoft YaHei. Verify each permitted font against its manifest hash. Excluded Noto
CJK must not be copied or embedded. The test neither downloads nor commits font files.

```powershell
$env:CRAFT_FONTS_DIR = (Resolve-Path '..\craft-fonts').Path
$env:CRAFT_FONTS_REQUIRED = '1'
$env:PDFCRAFT_TEST_REQUIRE_CJK = '1'
Remove-Item Env:PDFCRAFT_SYSTEM_FONTS -ErrorAction SilentlyContinue
cargo test --locked -p pdfcraft-ui-egui --test windows_release_fonts -- --nocapture
$env:PDFCRAFT_SYSTEM_FONTS = '0'
cargo test --locked -p pdfcraft-ui-egui --test windows_release_fonts -- --nocapture
Remove-Item Env:PDFCRAFT_SYSTEM_FONTS
```

This test is opt-in because release input and installed fonts are not guaranteed on CI.
With the flag enabled, an unsuitable input or missing glyphs fail rather than appearing to
pass a fallback check. Separate processes matter because the installed-face locator caches
its result.

## Coverage and scope

The test derives non-ASCII glyphs from the complete Simplified Chinese catalog, checks a
Chinese PDF filename, and exercises proportional, monospace, medium and semibold families
with font orders `true, false, true`. It checks Latin/Japanese/Arabic/Hebrew retention,
nonempty layout metrics and consistent Chinese face metrics. The opt-out compares definitions
and family order with embedded-only definitions and confirms the original missing glyphs remain.

The supplement branch was rerun on Windows 11 with this exact input: enabled and opt-out
processes both passed, covering 785 catalog glyphs. Workspace tests reported 1,267 passed and
5 declared ignored; fmt, all-target workspace Clippy with warnings denied, assets, layering
(34 crates) and WASM compilation (30 crates) passed. This is not browser runtime acceptance.
The later 0.6.0 local integration covered 791 glyphs with a different release pin;
these results must not be conflated. Rerun when catalogs, font inputs or fallback order change.

No system fonts, screenshots, private PDFs, binaries or control tokens are included. Missing-font
machines, macOS/Linux/web runtime behavior and all-platform policy approval remain outside scope.
