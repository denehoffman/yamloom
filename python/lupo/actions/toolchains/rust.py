from __future__ import annotations
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
)

if TYPE_CHECKING:
    from collections.abc import Mapping, Sequence

__all__ = ['install_rust_tool', 'setup_rust']


def setup_rust(
    *,
    name: Ostrlike = None,
    version: str = 'v1',
    toolchain: Ostrlike = None,
    target: Ostrlike = None,
    components: Sequence[StringLike] | None = None,
    cache: Oboollike = None,
    cache_directories: Sequence[StringLike] | None = None,
    cache_workspaces: Sequence[StringLike] | None = None,
    cache_on_failure: Oboollike = None,
    cache_key: Ostrlike = None,
    cache_shared_key: Ostrlike = None,
    cache_bin: Oboollike = None,
    cache_provider: Ostrlike = None,
    cache_all_crates: Oboollike = None,
    cache_workspace_crates: Oboollike = None,
    matcher: Oboollike = None,
    rustflags: Ostrlike = None,
    override: Oboollike = None,
    rust_src_dir: Ostrlike = None,
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
        'toolchain': toolchain,
        'target': target,
        'components': ','.join(str(s) for s in components)
        if components is not None
        else None,
        'cache': cache,
        'cache-directories': '\n'.join(str(s) for s in cache_directories)
        if cache_directories is not None
        else None,
        'cache-workspaces': '\n'.join(str(s) for s in cache_workspaces)
        if cache_workspaces is not None
        else None,
        'cache-on-failure': cache_on_failure,
        'cache-key': cache_key,
        'cache-shared-key': cache_shared_key,
        'cache-bin': cache_bin,
        'cache-provider': validate_choice(
            'cache_provider', cache_provider, ['github', 'buildjet', 'warpbuild']
        ),
        'cache-all-crates': cache_all_crates,
        'cache-workspace-crates': cache_workspace_crates,
        'matcher': matcher,
        'rustflags': rustflags,
        'override': override,
        'rust-src-dir': rust_src_dir,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Rust'

    return action(
        name,
        'actions-rust-lang/setup-rust-toolchain',
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


def install_rust_tool(
    *,
    tool: Sequence[StringLike],
    name: Ostrlike = None,
    version: str = 'v1',
    checksum: Oboollike = None,
    fallback: Ostrlike = None,
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
    if not tool:
        msg = "at least one 'tool' must be specified"
        raise ValueError(msg)

    options: dict[str, object] = {
        'tool': ','.join(str(s) for s in tool),
        'checksum': checksum,
        'fallback': validate_choice(
            'fallback', fallback, ['none', 'cargo-binstall', 'cargo-install']
        ),
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        suffix = 's' if len(tool) > 1 else ''
        name = f'Install Rust Tool{suffix}'

    return action(
        name,
        'taiki-e/install-action',
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
