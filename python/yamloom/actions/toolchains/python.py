from __future__ import annotations
from yamloom.actions.utils import validate_choice

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
    StringOrBoolLike,
)

if TYPE_CHECKING:
    from collections.abc import Mapping, Sequence

__all__ = ['setup_python', 'setup_uv']


def setup_python(
    *,
    name: Ostrlike = None,
    version: str = 'v6',
    python_version: StringLike | None = None,
    python_version_file: Ostrlike = None,
    check_latest: Oboollike = None,
    architecture: Ostrlike = None,
    token: Ostrlike = None,
    cache: Ostrlike = None,
    package_manager_cache: Oboollike = None,
    cache_dependency_path: Ostrlike = None,
    update_environment: Oboollike = None,
    allow_prereleases: Oboollike = None,
    freethreaded: Oboollike = None,
    pip_versions: Ostrlike = None,
    pip_install: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    working_directory: Ostrlike = None,
    shell: Ostr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'python-version': python_version,
        'python-version-file': python_version_file,
        'check-latest': check_latest,
        'architecture': architecture,
        'token': token,
        'cache': cache,
        'package-manager-cache': package_manager_cache,
        'cache-dependency-path': cache_dependency_path,
        'update-environment': update_environment,
        'allow-prereleases': allow_prereleases,
        'freethreaded': freethreaded,
        'pip-versions': pip_versions,
        'pip-install': pip_install,
    }

    if cache is not None:
        if isinstance(cache, str):
            lowered = cache.lower()
            if lowered not in {'pip', 'pipenv', 'poetry'}:
                msg = "'cache' must be 'pip', 'pipenv' or 'poetry'"
                raise ValueError(msg)
            options['cache'] = lowered
        else:
            options['cache'] = cache

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Python'

    return action(
        name,
        'actions/setup-python',
        ref=version,
        with_opts=options or None,
        args=args,
        entrypoint=entrypoint,
        condition=condition,
        working_directory=working_directory,
        shell=shell,
        id=id,
        env=env,
        continue_on_error=continue_on_error,
        timeout_minutes=timeout_minutes,
    )


def setup_uv(
    *,
    name: Ostrlike = None,
    version: str = 'v7',
    uv_version: Ostrlike = None,
    uv_version_file: Ostrlike = None,
    resolution_strategy: Ostrlike = None,
    python_version: Ostrlike = None,
    activate_environment: Oboollike = None,
    uv_working_directory: Ostrlike = None,
    checksum: Ostrlike = None,
    github_token: Ostrlike = None,
    enable_cache: StringOrBoolLike | None = None,
    cache_dependency_glob: Sequence[StringLike] | None = None,
    restore_cache: Oboollike = None,
    save_cache: Oboollike = None,
    cache_suffix: Ostrlike = None,
    cache_local_path: Ostrlike = None,
    prune_cache: Oboollike = None,
    cache_python: Oboollike = None,
    ignore_nothing_to_cache: Oboollike = None,
    ignore_empty_workdir: Oboollike = None,
    tool_dir: Ostrlike = None,
    tool_bin_dir: Ostrlike = None,
    manifest_file: Ostrlike = None,
    add_problem_matchers: Oboollike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    working_directory: Ostrlike = None,
    shell: Ostr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'version': uv_version,
        'version-file': uv_version_file,
        'resolution-strategy': validate_choice(
            'resolution_strategy', resolution_strategy, ['highest', 'lowest']
        ),
        'python-version': python_version,
        'activate-environment': activate_environment,
        'working-directory': uv_working_directory,
        'checksum': checksum,
        'github-token': github_token,
        'enable-cache': enable_cache,
        'cache-dependency-glob': '\n'.join(str(s) for s in cache_dependency_glob)
        if cache_dependency_glob is not None
        else None,
        'restore-cache': restore_cache,
        'save-cache': save_cache,
        'cache-suffix': cache_suffix,
        'cache-local-path': cache_local_path,
        'prune-cache': prune_cache,
        'cache-python': cache_python,
        'ignore-nothing-to-cache': ignore_nothing_to_cache,
        'ignore-empty-workdir': ignore_empty_workdir,
        'tool-dir': tool_dir,
        'tool-bin-dir': tool_bin_dir,
        'manifest-file': manifest_file,
        'add-problem-matchers': add_problem_matchers,
    }

    if enable_cache is not None:
        if isinstance(enable_cache, str):
            if enable_cache.lower() != 'auto':
                msg = "'enable_cache' must be 'auto', true or false"
                raise ValueError(msg)
            options['enable-cache'] = 'auto'
        elif isinstance(enable_cache, bool):
            options['enable-cache'] = enable_cache
        else:
            options['enable-cache'] = enable_cache

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup uv'

    return action(
        name,
        'astral-sh/setup-uv',
        ref=version,
        with_opts=options or None,
        args=args,
        entrypoint=entrypoint,
        condition=condition,
        working_directory=working_directory,
        shell=shell,
        id=id,
        env=env,
        continue_on_error=continue_on_error,
        timeout_minutes=timeout_minutes,
    )
