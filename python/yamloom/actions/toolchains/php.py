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

__all__ = ['setup_php']


def setup_php(
    *,
    name: Ostrlike = None,
    version: str = 'v2',
    php_version: Ostrlike = None,
    php_version_file: Ostrlike = None,
    extensions: Ostrlike = None,
    ini_file: Ostrlike = None,
    ini_values: Ostrlike = None,
    coverage: Ostrlike = None,
    tools: Ostrlike = None,
    github_token: Ostrlike = None,
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
        'php-version': php_version,
        'php-version-file': php_version_file,
        'extensions': extensions,
        'ini-file': validate_choice(
            'ini_file', ini_file, ['production', 'development', 'none']
        ),
        'ini-values': ini_values,
        'coverage': validate_choice('coverage', coverage, ['xdebug', 'pcov', 'none']),
        'tools': tools,
        'github-token': github_token,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup PHP'

    return action(
        name,
        'shivammathur/setup-php',
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
