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
)

if TYPE_CHECKING:
    from collections.abc import Mapping

__all__ = ['setup_ruby']


def setup_ruby(
    *,
    name: Ostrlike = None,
    version: str = 'v1',
    ruby_version: Ostrlike = None,
    rubygems: Ostrlike = None,
    bundler: Ostrlike = None,
    bundler_cache: Oboollike = None,
    ruby_working_directory: Ostrlike = None,
    cache_version: Ostrlike = None,
    self_hosted: Oboollike = None,
    windows_toolchain: Ostrlike = None,
    token: Ostrlike = None,
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
        'ruby-version': ruby_version,
        'rubygems': rubygems,
        'bundler': bundler,
        'bundler-cache': bundler_cache,
        'working-directory': ruby_working_directory,
        'cache-version': cache_version,
        'self-hosted': self_hosted,
        'windows-toolchain': validate_choice(
            'windows-toolchain', windows_toolchain, ['defauult', 'none']
        ),
        'token': token,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Ruby'

    return action(
        name,
        'actions/setup-ruby',
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
