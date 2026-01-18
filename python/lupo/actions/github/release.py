from __future__ import annotations
from lupo.actions.utils import check_string

from typing import TYPE_CHECKING

from ..._lupo import Step
from ..._lupo import action
from ..types import (
    Obool,
    Oboollike,
    Oboolstr,
    Ointlike,
    Ostr,
    Ostrlike,
    StringLike,
)

if TYPE_CHECKING:
    from collections.abc import Mapping

__all__ = ['release']


def release(
    *,
    name: Ostrlike = None,
    version: str = 'v2',
    body: Ostr = None,
    body_path: Ostr = None,
    draft: Obool = None,
    prerelease: Obool = None,
    preserve_order: Obool = None,
    files: Ostr = None,
    overwrite_files: Obool = None,
    release_name: Ostr = None,
    tag_name: Ostr = None,
    fail_on_unmatched_files: Obool = None,
    repository: Ostr = None,
    target_commitish: Ostr = None,
    token: Ostr = None,
    discussion_category_name: Ostr = None,
    generate_release_notes: Obool = None,
    append_body: Obool = None,
    make_latest: Ostr = None,
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
        'body': body,
        'body_path': body_path,
        'draft': draft,
        'prerelease': prerelease,
        'preserve_order': preserve_order,
        'files': files,
        'overwrite_files': overwrite_files,
        'name': release_name,
        'tag_name': tag_name,
        'fail_on_unmatched_files': fail_on_unmatched_files,
        'repository': repository,
        'target_commitish': target_commitish,
        'token': token,
        'discussion_category_name': discussion_category_name,
        'generate_release_notes': generate_release_notes,
        'append_body': append_body,
        'make_latest': make_latest,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        repository_str = check_string(options.get('repository'))
        if repository_str:
            name = f"Make Release for '{repository_str}'"
        else:
            name = 'Make Release'

    return action(
        name,
        'softprops/action-gh-release',
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
