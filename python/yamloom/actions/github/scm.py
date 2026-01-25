from __future__ import annotations
from yamloom.actions.utils import check_string

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

__all__ = ['checkout']


def checkout(
    *,
    name: Ostrlike = None,
    version: str = 'v6',
    repository: Ostrlike = None,
    ref: Ostrlike = None,
    token: Ostrlike = None,
    ssh_key: Ostrlike = None,
    ssh_known_hosts: Ostrlike = None,
    ssh_strict: Oboollike = None,
    ssh_user: Ostrlike = None,
    persist_credentials: Oboollike = None,
    path: Ostrlike = None,
    clean: Oboollike = None,
    filter: Ostrlike = None,  # noqa: A002
    sparse_checkout: Ostrlike = None,
    sparse_checkout_cone_mode: Oboollike = None,
    fetch_depth: Ointlike = None,
    fetch_tags: Oboollike = None,
    show_progress: Oboollike = None,
    lfs: Oboollike = None,
    submodules: Oboollike = None,
    get_safe_directory: Oboollike = None,
    github_server_url: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'repository': repository,
        'ref': ref,
        'token': token,
        'ssh-key': ssh_key,
        'ssh-known-hosts': ssh_known_hosts,
        'ssh-strict': ssh_strict,
        'ssh-user': ssh_user,
        'persist-credentials': persist_credentials,
        'path': path,
        'clean': clean,
        'filter': filter,
        'sparse-checkout': sparse_checkout,
        'sparse-checkout-cone-mode': sparse_checkout_cone_mode,
        'fetch-depth': fetch_depth,
        'fetch-tags': fetch_tags,
        'show-progress': show_progress,
        'lfs': lfs,
        'submodules': submodules,
        'get-safe-directory': get_safe_directory,
        'github-server-url': github_server_url,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        repository_str = check_string(options.get('repository'))
        if repository_str:
            name = f"Checkout '{repository_str}'"
        else:
            name = 'Checkout Repository'

    return action(
        name,
        'actions/checkout',
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
