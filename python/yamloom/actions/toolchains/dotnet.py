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

__all__ = ['setup_dotnet']


def setup_dotnet(
    *,
    name: Ostrlike = None,
    version: str = 'v5',
    dotnet_version: Ostrlike = None,
    dotnet_quality: Ostrlike = None,
    global_json_file: Ostrlike = None,
    source_url: Ostrlike = None,
    owner: Ostrlike = None,
    config_file: Ostrlike = None,
    cache: Oboollike = None,
    cache_dependency_path: Ostrlike | list[StringLike] = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'dotnet-version': dotnet_version,
        'dotnet-quality': validate_choice(
            'dotnet-quality',
            dotnet_quality,
            ['daily', 'signed', 'validated', 'preview', 'ga'],
        ),
        'global-json-file': global_json_file,
        'source-url': source_url,
        'owner': owner,
        'config-file': config_file,
        'cache': cache,
        'cache-dependency-path': cache_dependency_path,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup .NET'

    return action(
        name,
        'actions/setup-dotnet',
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
