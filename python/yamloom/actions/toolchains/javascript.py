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

__all__ = ['setup_bun']


def setup_bun(
    *,
    name: Ostrlike = None,
    version: str = 'v2',
    bun_version: Ostrlike = None,
    bun_version_file: Ostrlike = None,
    bun_download_url: Ostrlike = None,
    registry_url: Ostrlike = None,
    scope: Ostrlike = None,
    no_cache: Oboollike = None,
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
        'bun-version': bun_version,
        'bun-version-file': bun_version_file,
        'bun-download-url': bun_download_url,
        'registry-url': registry_url,
        'scope': scope,
        'no-cache': no_cache,
        'token': token,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Bun'

    return action(
        name,
        'oven-sh/setup-bun',
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
