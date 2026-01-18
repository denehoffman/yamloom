from __future__ import annotations
from collections.abc import Sequence
from lupo.actions.utils import validate_choice

from typing import TYPE_CHECKING

from ..._lupo import Step
from ..._lupo import action
from ..types import (
    Oboollike,
    Oboolstr,
    Ointlike,
    Ostr,
    Ostrlike,
    StringLike,
    BoolLike,
)

if TYPE_CHECKING:
    from collections.abc import Mapping

__all__ = ['setup_node', 'setup_pnpm']


def setup_node(
    *,
    name: Ostrlike = None,
    version: str = 'v6',
    node_version: Ostrlike = None,
    node_version_file: Ostrlike = None,
    check_latest: Oboollike = None,
    architecture: Ostrlike = None,
    token: Ostrlike = None,
    cache: Ostrlike = None,
    package_manager_cache: Oboollike = None,
    cache_dependency_path: Ostrlike = None,
    registry_url: Ostrlike = None,
    scope: Ostrlike = None,
    mirror: Ostrlike = None,
    mirror_token: Ostrlike = None,
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
        'node-version': node_version,
        'node-version-file': node_version_file,
        'check-latest': check_latest,
        'architecture': architecture,
        'token': token,
        'cache': validate_choice('cache', cache, ['npm', 'yarn', 'pnpm']),
        'package-manager-cache': package_manager_cache,
        'cache-dependency-path': cache_dependency_path,
        'registry-url': registry_url,
        'scope': scope,
        'mirror': mirror,
        'mirror-token': mirror_token,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Node'

    return action(
        name,
        'actions/setup-node',
        ref=version,
        with_opts=options,
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


def setup_pnpm(
    *,
    name: Ostrlike = None,
    version: str = 'v4',
    pnpm_version: Ostrlike = None,
    dest: Ostrlike = None,
    run_install: StringLike | BoolLike | None = None,
    cache: Oboollike = None,
    cache_dependency_path: Ostrlike | Sequence[StringLike] = None,
    package_json_file: Ostrlike = None,
    standalone: Oboollike = None,
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
        'version': pnpm_version,
        'dest': dest,
        'run_install': run_install,
        'cache': cache,
        'cache_dependency_path': cache_dependency_path,
        'package_json_file': package_json_file,
        'standalone': standalone,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup pnpm'

    return action(
        name,
        'pnpm/action-setup',
        ref=version,
        with_opts=options,
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
