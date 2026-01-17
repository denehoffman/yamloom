from __future__ import annotations

from typing import TYPE_CHECKING, TypeAlias

from ..._lupo import Step
from ..._lupo import action as _action
from ...expressions import BooleanExpression, NumberExpression, StringExpression

if TYPE_CHECKING:
    from collections.abc import Mapping

Ostr: TypeAlias = str | None
Obool: TypeAlias = bool | None
Oint: TypeAlias = int | None
StringLike: TypeAlias = str | StringExpression
BoolLike: TypeAlias = bool | BooleanExpression
IntLike: TypeAlias = int | NumberExpression
Ostrlike: TypeAlias = StringLike | None
Oboolstr: TypeAlias = BooleanExpression | str | None
Oboollike: TypeAlias = BoolLike | None
Ointlike: TypeAlias = IntLike | None

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
    working_directory: Ostrlike = None,
    shell: Ostr = None,
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

    return _action(
        name,
        'actions/setup-go',
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
