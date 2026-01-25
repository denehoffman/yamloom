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

__all__ = ['create_pull_request']


def create_pull_request(
    *,
    name: Ostrlike = None,
    version: str = 'v8',
    token: Ostrlike = None,
    branch_token: Ostrlike = None,
    path: Ostrlike = None,
    add_paths: Ostrlike = None,
    commit_message: Ostrlike = None,
    committer: Ostrlike = None,
    author: Ostrlike = None,
    signoff: Oboollike = None,
    branch: Ostrlike = None,
    delete_branch: Oboollike = None,
    branch_suffix: Ostrlike = None,
    base: Ostrlike = None,
    push_to_fork: Ostrlike = None,
    sign_commits: Oboollike = None,
    title: Ostrlike = None,
    body: Ostrlike = None,
    body_path: Ostrlike = None,
    labels: Ostrlike = None,
    assignees: Ostrlike = None,
    reviewers: Ostrlike = None,
    team_reviewers: Ostrlike = None,
    milestone: Ointlike = None,
    draft: Oboollike = None,
    maintainer_can_modify: Oboollike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'token': token,
        'branch-token': branch_token,
        'path': path,
        'add-paths': add_paths,
        'commit-message': commit_message,
        'committer': committer,
        'author': author,
        'signoff': signoff,
        'branch': branch,
        'delete-branch': delete_branch,
        'branch-suffix': validate_choice(
            'branch_suffix', branch_suffix, ['random', 'timestamp', 'short-commit-hash']
        ),
        'base': base,
        'push-to-fork': push_to_fork,
        'sign-commits': sign_commits,
        'title': title,
        'body': body,
        'body-path': body_path,
        'labels': labels,
        'assignees': assignees,
        'reviewers': reviewers,
        'team-reviewers': team_reviewers,
        'milestone': milestone,
        'draft': draft,
        'maintainer-can-modify': maintainer_can_modify,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Create Pull Request'

    return action(
        name,
        'peter-evans/create-pull-request',
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
