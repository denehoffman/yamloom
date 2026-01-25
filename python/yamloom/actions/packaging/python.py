from __future__ import annotations

from typing import TYPE_CHECKING

from ..._yamloom import Step
from ..._yamloom import action
from ..types import (
    Oboollike,
    Oboolstr,
    Ointlike,
    Ostr,
    Ostrlike,
    StringLike,
)

if TYPE_CHECKING:
    from collections.abc import Mapping

__all__ = ['maturin', 'pypi_publish']


def maturin(
    *,
    name: Ostrlike = None,
    version: str = 'v1',
    token: Ostrlike = None,
    command: Ostrlike = None,
    maturin_version: Ostrlike = None,
    manylinux: Ostrlike = None,
    container: Ostrlike = None,
    docker_options: Ostrlike = None,
    host_home_mount: Ostrlike = None,
    target: Ostrlike = None,
    rust_toolchain: Ostrlike = None,
    rustup_components: list[StringLike] | None = None,
    working_directory: Ostrlike = None,
    sccache: Oboollike = None,
    before_script_linux: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'command': command,
        'maturin-version': maturin_version,
        'manylinux': manylinux,
        'target': target,
        'container': container,
        'docker-options': docker_options,
        'rust-toolchain': rust_toolchain,
        'rustup-components': ','.join(str(s) for s in rustup_components)
        if rustup_components is not None
        else None,
        'working-directory': working_directory,
        'sccache': sccache,
        'before-script-linux': before_script_linux,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Maturin Action'

    return action(
        name,
        'PyO3/maturin-action',
        ref=version,
        with_opts=options or None,
        args=args,
        entrypoint=entrypoint,
        condition=condition,
        id=id,
        env=env,
        continue_on_error=continue_on_error,
        timeout_minutes=timeout_minutes,
    )


def pypi_publish(
    *,
    name: Ostrlike = None,
    version: str = 'release/v1',
    user: Ostrlike = None,
    password: Ostrlike = None,
    repository_url: Ostrlike = None,
    packages_dir: Ostrlike = None,
    verify_metadata: Oboollike = None,
    skip_existing: Oboollike = None,
    verbose: Oboollike = None,
    print_hash: Oboollike = None,
    attestations: Oboollike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'user': user,
        'password': password,
        'repository-url': repository_url,
        'packages-dir': packages_dir,
        'verify-metadata': verify_metadata,
        'skip-existing': skip_existing,
        'verbose': verbose,
        'print-hash': print_hash,
        'attestations': attestations,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Publish to PyPI'

    return action(
        name,
        'pypa/gh-action-pypi-publish',
        ref=version,
        with_opts=options or None,
        args=args,
        entrypoint=entrypoint,
        condition=condition,
        id=id,
        env=env,
        continue_on_error=continue_on_error,
        timeout_minutes=timeout_minutes,
    )
