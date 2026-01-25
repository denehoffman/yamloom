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


__all__ = ['attest_build_provenance']


def attest_build_provenance(
    *,
    path: StringLike,
    name: Ostrlike = None,
    version: str = 'v3',
    subject_path: Ostrlike = None,
    subject_digest: Ostrlike = None,
    subject_checksums: Ostrlike = None,
    subject_name: Ostrlike = None,
    push_to_registry: Oboollike = None,
    create_storage_record: Oboollike = None,
    show_summary: Oboollike = None,
    github_token: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    if (
        sum(
            value is not None
            for value in (subject_path, subject_digest, subject_checksums)
        )
        != 1
    ):
        raise ValueError(
            'Exactly one of subject_path, subject_digest, or subject_checksums must be set'
        )

    options: dict[str, object] = {
        'subject-path': subject_path,
        'subject-digest': subject_digest,
        'subject-checksums': subject_checksums,
        'subject-name': subject_name,
        'push-to-registry': push_to_registry,
        'create-storage-record': create_storage_record,
        'show-summary': show_summary,
        'github-token': github_token,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Create Attestation'

    return action(
        name,
        'actions/attest-build-provenance',
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
