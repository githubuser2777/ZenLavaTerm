## Phase 12 Integrity Violation: Native Audio Backends are Synthetic Fakes

**Phase 12 / V1.0 Release Blocker**

The native audio capture architectures (WASAPI, PipeWire, CoreAudio) implemented for Issues #47, #48, and #49 are synthetic mocks generating sine waves (`(phase * TAU).sin() * 0.35`). While the PR 57 reviewer requested explicit documentation that real hardware bindings are "deferred to v1.1," the remediation commit (`b092bbe`) failed to update `README.md`, `docs/audio.md`, and `docs/roadmap.md`.

The documentation currently falsely advertises working native hardware audio capture.

**Required Remediation:**
1. Update `README.md` to explicitly state that the `--audio` flag currently utilizes a synthetic audio generator, and that true hardware capture bindings (WASAPI, PipeWire, CoreAudio) are deferred to v1.1.
2. Update `docs/roadmap.md` to move "Native Audio Capture" out of Phase 12 and into Phase 13 (v1.1).
3. Update `docs/audio.md` to clarify the synthetic scope.
4. Push these documentation corrections to the `pr57` branch.
5. Merge PR #57 once CI passes.

Additionally, `scripts/update_package_manifests.sh` is missing from the CI pipelines (`package.yml` and `release.yml`) and should be enforced to prevent missing checksum validations.
