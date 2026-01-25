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

__all__ = ['setup_java']


def setup_java(
    *,
    name: Ostrlike = None,
    version: str = 'v5',
    java_version: Ostrlike = None,
    java_version_file: Ostrlike = None,
    distribution: Ostrlike = None,
    java_package: Ostrlike = None,
    check_latest: Oboollike = None,
    architecture: Ostrlike = None,
    jdk_file: Ostrlike = None,
    cache: Ostrlike = None,
    cache_dependency_path: Ostrlike = None,
    overwrite_settings: Oboollike = None,
    server_id: Ostrlike = None,
    server_username: Ostrlike = None,
    server_password: Ostrlike = None,
    settings_path: Ostrlike = None,
    gpg_private_key: Ostrlike = None,
    gpg_passphrase: Ostrlike = None,
    mvn_toolchain_id: Ostrlike = None,
    mvn_toolchain_vendor: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'java-version': java_version,
        'java-version-file': java_version_file,
        'distribution': distribution,
        'java-package': validate_choice(
            'java_package', java_package, ['jdk', 'jre', 'jdk+fx', 'jre+fx']
        ),
        'check-latest': check_latest,
        'architecture': validate_choice(
            'architecture', architecture, ['x86', 'x64', 'armv7', 'aarch64', 'ppc64le']
        ),
        'jdkFile': jdk_file,
        'cache': validate_choice('cache', cache, ['maven', 'gradle', 'sbt']),
        'cache-dependency-path': cache_dependency_path,
        'overwrite-settings': overwrite_settings,
        'server-id': server_id,
        'server-username': server_username,
        'server-password': server_password,
        'settings-path': settings_path,
        'gpg-private_key': gpg_private_key,
        'gpg-passphrase': gpg_passphrase,
        'mvn-toolchain-id': mvn_toolchain_id,
        'mvn-toolchain-vendor': mvn_toolchain_vendor,
    }
    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        name = 'Setup Java'

    return action(
        name,
        'actions/setup-java',
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
