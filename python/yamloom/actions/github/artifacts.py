from __future__ import annotations
from yamloom.actions.utils import validate_choice, check_string

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

WARN_RETENTION_DAYS: int = 90
MAX_COMPRESSION_LEVEL: int = 9

__all__ = ['download_artifact', 'upload_artifact', 'upload_artifact_merge']


def upload_artifact(
    *,
    path: StringLike,
    name: Ostrlike = None,
    version: str = 'v6',
    artifact_name: Ostrlike = None,
    if_no_files_found: Ostrlike = None,
    retention_days: Ointlike = None,
    compression_level: Ointlike = None,
    overwrite: Oboollike = None,
    include_hidden_files: Oboollike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'path': path,
        'name': artifact_name,
        'if_no_files_found': validate_choice(
            'if_no_files_found', if_no_files_found, ['warn', 'error', 'ignore']
        ),
        'retention-days': retention_days,
        'compression-level': compression_level,
        'overwrite': overwrite,
        'include-hidden-files': include_hidden_files,
    }

    if retention_days is not None:
        if isinstance(retention_days, int) and not isinstance(retention_days, bool):
            if retention_days < 1:
                msg = 'retention days must be > 0'
                raise ValueError(msg)
            if retention_days > WARN_RETENTION_DAYS:
                print(
                    f'Warning: retention days should be <= {WARN_RETENTION_DAYS} unless a higher limit is made in the repository settings!'
                )
        options['retention-days'] = retention_days

    if compression_level is not None:
        if (
            isinstance(compression_level, int)
            and not isinstance(compression_level, bool)
        ) and (compression_level < 0 or compression_level > MAX_COMPRESSION_LEVEL):
            msg = f'compression level must be in the range 0-{MAX_COMPRESSION_LEVEL}'
            raise ValueError(msg)
        options['compression-level'] = compression_level

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        artifact_str = check_string(options.get('artifact_name'))
        if artifact_str:
            name = f'Upload {artifact_str}'
        else:
            name = 'Upload Artifact'

    return action(
        name,
        'actions/upload-artifact',
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


def upload_artifact_merge(
    *,
    pattern: Ostrlike = None,
    name: Ostrlike = None,
    version: str = 'v6',
    artifact_name: Ostrlike = None,
    separate_directories: Oboollike = None,
    delete_merged: Oboollike = None,
    retention_days: Ointlike = None,
    compression_level: Ointlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'name': artifact_name,
        'pattern': pattern,
        'separate-directories': separate_directories,
        'delete-merged': delete_merged,
        'retention-days': retention_days,
        'compression-level': compression_level,
    }

    if retention_days is not None:
        if isinstance(retention_days, int) and not isinstance(retention_days, bool):
            if retention_days < 1:
                msg = 'retention days must be > 0'
                raise ValueError(msg)
            if retention_days > WARN_RETENTION_DAYS:
                print(
                    f'Warning: retention days should be <= {WARN_RETENTION_DAYS} unless a higher limit is made in the repository settings!'
                )
        options['retention-days'] = retention_days

    if compression_level is not None:
        if (
            isinstance(compression_level, int)
            and not isinstance(compression_level, bool)
        ) and (compression_level < 0 or compression_level > MAX_COMPRESSION_LEVEL):
            msg = f'compression level must be in the range 0-{MAX_COMPRESSION_LEVEL}'
            raise ValueError(msg)
        options['compression-level'] = compression_level

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        artifact_str = check_string(options.get('artifact_name'))
        if artifact_str:
            name = f'Upload (merged) {artifact_str}'
        else:
            name = 'Upload (merged) Artifact'

    return action(
        name,
        'actions/upload-artifact/merge',
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


def download_artifact(
    *,
    path: Ostrlike = None,
    name: Ostrlike = None,
    version: str = 'v7',
    artifact_name: Ostrlike = None,
    artifact_ids: list[StringLike] | None = None,
    pattern: Ostrlike = None,
    merge_multiple: Oboollike = None,
    github_token: Ostrlike = None,
    repository: Ostrlike = None,
    run_id: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'name': artifact_name,
        'artifact-ids': ','.join(str(s) for s in artifact_ids)
        if artifact_ids is not None
        else None,
        'pattern': pattern,
        'path': path,
        'merge-multiple': merge_multiple,
        'github-token': github_token,
        'repository': repository,
        'run-id': run_id,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        artifact_str = check_string(options.get('artifact_name'))
        if artifact_str:
            name = f'Download {artifact_str}'
        else:
            name = 'Download Artifact'

    return action(
        name,
        'actions/download-artifact',
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
