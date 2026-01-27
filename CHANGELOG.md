# Changelog

## [0.5.1](https://github.com/denehoffman/yamloom/compare/yamloom-v0.5.0...yamloom-v0.5.1) (2026-01-27)


### Bug Fixes

* Update output and types to reflect action.ymls ([10e8cb8](https://github.com/denehoffman/yamloom/commit/10e8cb86236d9553a6429ecf3d43c378d3096d5a))

## [0.5.0](https://github.com/denehoffman/yamloom/compare/yamloom-v0.4.1...yamloom-v0.5.0) (2026-01-27)


### ⚠ BREAKING CHANGES

* This renames all actions to CapWords/UpperCamelCase, so it breaks pretty much every workflow

### Features

* Add option to skip recommended permissions and add logic to skip them anyway if not using GITHUB_TOKEN ([e9a6219](https://github.com/denehoffman/yamloom/commit/e9a6219528e1028e01d1acd1758456d64ce993ed))
* Consolidate actions and outputs into single classes rather than functions, add recommended permissions which automatically accumulate per job ([7e68334](https://github.com/denehoffman/yamloom/commit/7e683346ccf1896dfea28eccc3117829bb4dc2e7))


### Bug Fixes

* Rename SetupUv to SetupUV ([8b96649](https://github.com/denehoffman/yamloom/commit/8b9664979726f632ea6939aa751e0f27240addf3))

## [0.4.1](https://github.com/denehoffman/yamloom/compare/yamloom-v0.4.0...yamloom-v0.4.1) (2026-01-26)


### Features

* Add schema validation to workflows ([f9e8eb8](https://github.com/denehoffman/yamloom/commit/f9e8eb8f3f1420898f6abed9a198b321c40cb00a))


### Bug Fixes

* Stop round-trip YAML validation by mapping Yaml::Real properly, add sublicenses for schemastore ([a5cbf69](https://github.com/denehoffman/yamloom/commit/a5cbf69bb9706a02c1eebaa630dbbcf5e42fadeb))

## [0.4.0](https://github.com/denehoffman/yamloom/compare/yamloom-v0.3.0...yamloom-v0.4.0) (2026-01-25)


### ⚠ BREAKING CHANGES

* If `working_directory` or `shell` were passed to an `action` builder, the resulting action would either be invalid or these options would be ignored. These kwargs are no longer allowed in any actions or derived a ctions.

### Features

* Add Output classes for each action which has outputs, document inputs, and remove/modify some inconsistent inputs ([b0e40c3](https://github.com/denehoffman/yamloom/commit/b0e40c3d8f32ff9fc2d5100b47c3f0ecf7994f6b))


### Bug Fixes

* Steps which specify actions no longer include working_directory or she ([efac7d1](https://github.com/denehoffman/yamloom/commit/efac7d19cdca69d9057909b57055f550c9c31c38))

## [0.3.0](https://github.com/denehoffman/yamloom/compare/yamloom-v0.2.3...yamloom-v0.3.0) (2026-01-25)


### ⚠ BREAKING CHANGES

* the signature for Job has been changed from Job(steps, *, *args) to Job(*, steps, *args), so this will break all scripts which construct Jobs (use keyword arguments in the future)

### Features

* Add context information to expressions to allow for runtime validation ([5d79381](https://github.com/denehoffman/yamloom/commit/5d7938127753cc1b17704c0fc3d2e471ba33952f))


### Bug Fixes

* Update CI script to use new kwarg for ([952e8f6](https://github.com/denehoffman/yamloom/commit/952e8f6ae96a3efeb131095f0d1969b06354a116))
* Validate runs-on vs uses for Jobs, require steps to be kw-only ([4ef76fa](https://github.com/denehoffman/yamloom/commit/4ef76fa6aa73e4bea44e2f6fc5e144566a40abde))

## [0.2.3](https://github.com/denehoffman/yamloom/compare/yamloom-v0.2.2...yamloom-v0.2.3) (2026-01-23)


### Bug Fixes

* **ci/coverage:** Remove unused path argument ([f37b09e](https://github.com/denehoffman/yamloom/commit/f37b09ed45123e78e82d2c296a9004ebbb328f3c))
* **ci:** Add __init__.py so coverage subpackage is discoverable ([40dcc08](https://github.com/denehoffman/yamloom/commit/40dcc087ae5741b387c815751a8b7191df60b4a0))

## [0.2.2](https://github.com/denehoffman/yamloom/compare/yamloom-v0.2.1...yamloom-v0.2.2) (2026-01-22)


### Bug Fixes

* **toolchains/rust:** Update version on taike-e/install-action ([8308450](https://github.com/denehoffman/yamloom/commit/83084502b36415a7f188339dd8ffcdb16528f2af))

## [0.2.1](https://github.com/denehoffman/yamloom/compare/yamloom-v0.2.0...yamloom-v0.2.1) (2026-01-21)


### Bug Fixes

* **github/release:** Correct typo in release-please action ([fec00a0](https://github.com/denehoffman/yamloom/commit/fec00a0217f696b0b24d5cffc6c2ccd708ec3a99))

## [0.2.0](https://github.com/denehoffman/yamloom/compare/yamloom-v0.1.1...yamloom-v0.2.0) (2026-01-21)


### ⚠ BREAKING CHANGES

* **release:** force release 0.2.0
* Update all calls from `script(name, x, y, z, **kwargs)` to `script(x, y, z, name=name, **kwargs)` (name is now optional)

### Features

* Add a pre-commit hook which finds a custom lupo sync script made by the user ([5e2f976](https://github.com/denehoffman/yamloom/commit/5e2f9760bb10de8466ae74db76fae7dcfd25eb4b))
* **ci/coverage.py:** Add Codecov action ([b9a1199](https://github.com/denehoffman/yamloom/commit/b9a1199366a16e7e6edf3a627d179f64adf191d1))
* First commit, need to finish contexts ([59c8dcb](https://github.com/denehoffman/yamloom/commit/59c8dcbdc6de6ae843a6a118c6d44c187918e8ef))
* **github/attest.py:** Add attestation action ([5e46e94](https://github.com/denehoffman/yamloom/commit/5e46e94e9b33aa7fc9349dcbba0569b3003e3149))
* **github/pull_request.py:** Add action to create PRs ([311090e](https://github.com/denehoffman/yamloom/commit/311090e953aaa9dde61fcf2adfe52c3ad9e2b65d))
* **github/release.py:** Add release-please action ([548c756](https://github.com/denehoffman/yamloom/commit/548c756e22f28902fd9fe59b425493b7f3a0e2b8))
* Make expressions/context all nice, improve syntax in various places, and some minor fixes ([98e43c7](https://github.com/denehoffman/yamloom/commit/98e43c7fbc7711e2a1fe75968bdc73982201a2b2))
* Move actions into the Python API rather than writing a bunch of Rust code. ([f08a453](https://github.com/denehoffman/yamloom/commit/f08a4531ea7494a8b4dd4eb18d6c62fa78d1ddbb))
* **packaging/python.py:** Add PyPI publishing action ([f3ad401](https://github.com/denehoffman/yamloom/commit/f3ad401eec859b0043663411c01e30a0089b58d7))
* Reorganize the python library structure in preparation for future premade actions ([5d01513](https://github.com/denehoffman/yamloom/commit/5d01513d0b229ea0b98a1b12d16472b3d7220ef7))
* Scripts no longer require a name as their first argument (and names are optional on actions but recommended) ([47465a7](https://github.com/denehoffman/yamloom/commit/47465a7f8633377a1657db7b39bc062e7c27ee70))
* Tentative implementation of expressions ([afb113d](https://github.com/denehoffman/yamloom/commit/afb113d4a6487620567de1de4d90671d973ec28d))
* **toolchains/javascript.py:** Add setup_bun action ([2602ea1](https://github.com/denehoffman/yamloom/commit/2602ea1c30ab3758f44da971172f7aa8cfda669a))
* **Workflow:** Add `dump` method to write `Workflow` to file (overwrite by default) ([723f38d](https://github.com/denehoffman/yamloom/commit/723f38d2743d60f251ff7a54cfffa7209063dfcf))


### Bug Fixes

* A few changes to type stubs and scripts now consume all non-kwargs to make multiline scripts easier to read/write ([3a894d4](https://github.com/denehoffman/yamloom/commit/3a894d42d9c42f822139f41fe87fec178eb034c1))
* A few updates to make sure with isn't an empty dict, and use the proper repo for the MPI action ([56d874a](https://github.com/denehoffman/yamloom/commit/56d874ab2afe261c08f6b1bc419570b117a03b79))
* Add README+LICENSE, make yamloom entrypoint, update escape handling to only activate on Reals, add test step to workflow, remove hook for yamloom, use list instead of Sequence for type hints ([4cf29f7](https://github.com/denehoffman/yamloom/commit/4cf29f7afd55565fb84b6d5e7b69fec077a13302))
* Change minimum Python version to 3.9, override __getattr__ alongside __getitem__ in expressions, rename starts_with and ends_with to startswith and endswith to match Python str implementations ([6a1d60d](https://github.com/denehoffman/yamloom/commit/6a1d60d54481c726f9f3d28aa3691088f9c366ba))
* **ci:** Force release-please to bump versions properly ([1686880](https://github.com/denehoffman/yamloom/commit/16868809460969119bab10ad80c21a728df4b12f))
* **ci:** Use proper release type (rust) and fix path issues ([a4e1e8e](https://github.com/denehoffman/yamloom/commit/a4e1e8e91112e0148b73e0a27839d6e38d3eacfb))
* Properly escape newlines, tabs, and other sequences in raw string outputs ([d69a120](https://github.com/denehoffman/yamloom/commit/d69a120962e93adfd15f6a785ed0f2c09c4bab38))
* **setup_python:** Change python_version argument to no longer accept arrays ([6edc613](https://github.com/denehoffman/yamloom/commit/6edc613fd4d15f5b1e879bf3377bcb465f2d0cb5))


### Miscellaneous Chores

* Release 0.2.0 ([d02519c](https://github.com/denehoffman/yamloom/commit/d02519c4df505c19436cecdb29613bc7437d7ad0))
* **release:** Trigger release 0.2.0 ([40aacf0](https://github.com/denehoffman/yamloom/commit/40aacf0a7d9572eaec16f11212a78368c3e56ca9))
* Trigger release ([86c7264](https://github.com/denehoffman/yamloom/commit/86c7264d0b7dc575337625f0d68e421033f60011))

## [0.1.1](https://github.com/denehoffman/yamloom/compare/v0.1.0...v0.1.1) (2026-01-20)


### Bug Fixes

* Add README+LICENSE, make yamloom entrypoint, update escape handling to only activate on Reals, add test step to workflow, remove hook for yamloom, use list instead of Sequence for type hints ([4cf29f7](https://github.com/denehoffman/yamloom/commit/4cf29f7afd55565fb84b6d5e7b69fec077a13302))


### Miscellaneous Chores

* Trigger release ([86c7264](https://github.com/denehoffman/yamloom/commit/86c7264d0b7dc575337625f0d68e421033f60011))

## 0.1.0 (2026-01-19)


### Features

* Add a pre-commit hook which finds a custom lupo sync script made by the user ([5e2f976](https://github.com/denehoffman/yamloom/commit/5e2f9760bb10de8466ae74db76fae7dcfd25eb4b))
* First commit, need to finish contexts ([59c8dcb](https://github.com/denehoffman/yamloom/commit/59c8dcbdc6de6ae843a6a118c6d44c187918e8ef))
* Make expressions/context all nice, improve syntax in various places, and some minor fixes ([98e43c7](https://github.com/denehoffman/yamloom/commit/98e43c7fbc7711e2a1fe75968bdc73982201a2b2))
* Move actions into the Python API rather than writing a bunch of Rust code. ([f08a453](https://github.com/denehoffman/yamloom/commit/f08a4531ea7494a8b4dd4eb18d6c62fa78d1ddbb))
* Reorganize the python library structure in preparation for future premade actions ([5d01513](https://github.com/denehoffman/yamloom/commit/5d01513d0b229ea0b98a1b12d16472b3d7220ef7))
* Tentative implementation of expressions ([afb113d](https://github.com/denehoffman/yamloom/commit/afb113d4a6487620567de1de4d90671d973ec28d))
* **Workflow:** Add `dump` method to write `Workflow` to file (overwrite by default) ([723f38d](https://github.com/denehoffman/yamloom/commit/723f38d2743d60f251ff7a54cfffa7209063dfcf))


### Bug Fixes

* A few changes to type stubs and scripts now consume all non-kwargs to make multiline scripts easier to read/write ([3a894d4](https://github.com/denehoffman/yamloom/commit/3a894d42d9c42f822139f41fe87fec178eb034c1))
* A few updates to make sure with isn't an empty dict, and use the proper repo for the MPI action ([56d874a](https://github.com/denehoffman/yamloom/commit/56d874ab2afe261c08f6b1bc419570b117a03b79))
* Change minimum Python version to 3.9, override __getattr__ alongside __getitem__ in expressions, rename starts_with and ends_with to startswith and endswith to match Python str implementations ([6a1d60d](https://github.com/denehoffman/yamloom/commit/6a1d60d54481c726f9f3d28aa3691088f9c366ba))
* Properly escape newlines, tabs, and other sequences in raw string outputs ([d69a120](https://github.com/denehoffman/yamloom/commit/d69a120962e93adfd15f6a785ed0f2c09c4bab38))
* **setup_python:** Change python_version argument to no longer accept arrays ([6edc613](https://github.com/denehoffman/yamloom/commit/6edc613fd4d15f5b1e879bf3377bcb465f2d0cb5))
