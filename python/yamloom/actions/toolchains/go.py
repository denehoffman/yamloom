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

__all__ = ['setup_go']


def setup_go(
    *,
    name: Ostrlike = None,
    version: str = 'v6',
    go_version: Ostrlike = None,
    go_version_file: Ostrlike = None,
    check_latest: Oboollike = None,
    architecture: Ostrlike = None,
    token: Ostrlike = None,
    cache: Oboollike = None,
    cache_dependency_path: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'go-version': go_version,
        'go-version-file': go_version_file,
        'check-latest': check_latest,
        'architecture': architecture,
        'token': token,
        'cache': cache,
        'cache-dependency-path': cache_dependency_path,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Go'

    return action(
        name,
        'actions/setup-go',
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
