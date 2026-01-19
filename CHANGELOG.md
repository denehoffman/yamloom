# Changelog

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
