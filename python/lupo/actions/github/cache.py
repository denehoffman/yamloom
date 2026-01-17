from __future__ import annotations

from typing import TYPE_CHECKING, TypeAlias

from ..._lupo import Step
from ..._lupo import action
from ...expressions import BooleanExpression, NumberExpression, StringExpression

if TYPE_CHECKING:
    from collections.abc import Mapping, Sequence

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

__all__ = ['cache', 'cache_restore', 'cache_save']


def cache(
    *,
    key: str,
    name: Ostrlike = None,
    version: str = 'v5',
    path: Sequence[str] | None = None,
    restore_keys: Sequence[str] | None = None,
    enable_cross_os_archive: Obool = None,
    fail_on_cache_miss: Obool = None,
    lookup_only: Obool = None,
    segment_download_timeout_mins: Oint = None,
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
        'key': key,
        'path': list(path) if path is not None else None,
        'restore-keys': list(restore_keys) if restore_keys is not None else None,
        'ensembleCrossOsArchive': enable_cross_os_archive,
        'fail-on-cache-miss': fail_on_cache_miss,
        'lookup-only': lookup_only,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Save/Restore Cache'

    if segment_download_timeout_mins is not None:
        merged_env = dict(env or {})
        merged_env['SEGMENT_DOWNLOAD_TIMEOUT_MINS'] = str(segment_download_timeout_mins)
        env = merged_env

    return action(
        name,
        'actions/cache',
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


def cache_save(
    *,
    key: str,
    name: Ostrlike = None,
    version: str = 'v5',
    path: Sequence[str] | None = None,
    restore_keys: Sequence[str] | None = None,
    enable_cross_os_archive: Obool = None,
    fail_on_cache_miss: Obool = None,
    lookup_only: Obool = None,
    segment_download_timeout_mins: Oint = None,
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
        'key': key,
        'path': list(path) if path is not None else None,
        'restore-keys': list(restore_keys) if restore_keys is not None else None,
        'ensembleCrossOsArchive': enable_cross_os_archive,
        'fail-on-cache-miss': fail_on_cache_miss,
        'lookup-only': lookup_only,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Save Cache'

    if segment_download_timeout_mins is not None:
        merged_env = dict(env or {})
        merged_env['SEGMENT_DOWNLOAD_TIMEOUT_MINS'] = str(segment_download_timeout_mins)
        env = merged_env

    return action(
        name,
        'actions/cache/save',
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


def cache_restore(
    *,
    key: StringLike,
    name: Ostrlike = None,
    version: str = 'v5',
    path: Sequence[StringLike] | None = None,
    restore_keys: Sequence[StringLike] | None = None,
    enable_cross_os_archive: Oboollike = None,
    fail_on_cache_miss: Oboollike = None,
    lookup_only: Oboollike = None,
    segment_download_timeout_mins: Ointlike = None,
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
        'key': key,
        'path': list(path) if path is not None else None,
        'restore-keys': list(restore_keys) if restore_keys is not None else None,
        'ensembleCrossOsArchive': enable_cross_os_archive,
        'fail-on-cache-miss': fail_on_cache_miss,
        'lookup-only': lookup_only,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Restore Cache'

    if segment_download_timeout_mins is not None:
        merged_env = dict(env or {})
        merged_env['SEGMENT_DOWNLOAD_TIMEOUT_MINS'] = str(segment_download_timeout_mins)
        env = merged_env

    return action(
        name,
        'actions/cache/restore',
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
