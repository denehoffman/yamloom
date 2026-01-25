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


__all__ = ['codecov']


def codecov(
    *,
    name: Ostrlike = None,
    version: str = 'v5',
    base_sha: Ostrlike = None,
    binary: Ostrlike = None,
    codecov_yml_path: Ostrlike = None,
    commit_parent: Ostrlike = None,
    directory: Ostrlike = None,
    disable_file_fixes: Oboollike = None,
    disable_search: Oboollike = None,
    disable_safe_directory: Oboollike = None,
    disable_telem: Oboollike = None,
    dry_run: Oboollike = None,
    env_vars: Ostrlike = None,
    exclude: Ostrlike = None,
    fail_ci_if_error: Oboollike = None,
    files: Ostrlike = None,
    flags: Ostrlike = None,
    force: Oboollike = None,
    git_service: Ostrlike = None,
    gcov_args: Ostrlike = None,
    gcov_executable: Ostrlike = None,
    gcov_ignore: Ostrlike = None,
    gcov_include: Ostrlike = None,
    handle_no_reports_found: Oboollike = None,
    job_code: Ostrlike = None,
    codecov_name: Ostrlike = None,
    network_filter: Ostrlike = None,
    network_prefix: Ostrlike = None,
    os: Ostrlike = None,  #
    override_branch: Ostrlike = None,
    override_build: Ointlike = None,
    override_build_url: Ostrlike = None,
    override_commit: Ostrlike = None,
    override_pr: Ointlike = None,
    plugins: Ostrlike = None,
    recurse_submodules: Oboollike = None,
    report_code: Ostrlike = None,
    report_type: Ostrlike = None,
    root_dir: Ostrlike = None,
    run_command: Ostrlike = None,
    skip_validation: Oboollike = None,
    slug: Ostrlike = None,
    swift_project: Ostrlike = None,
    token: Ostrlike = None,
    url: Ostrlike = None,
    use_legacy_upload_endpoint: Oboollike = None,
    use_oidc: Oboollike = None,
    use_pypi: Oboollike = None,
    verbose: Oboollike = None,
    codecov_version: Ostrlike = None,
    working_directory: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'base_sha': base_sha,
        'binary': binary,
        'codecov_yml_path': codecov_yml_path,
        'commit_parent': commit_parent,
        'directory': directory,
        'disable_file_fixes': disable_file_fixes,
        'disable_search': disable_search,
        'disable_safe_directory': disable_safe_directory,
        'disable_telem': disable_telem,
        'dry_run': dry_run,
        'env_vars': env_vars,
        'exclude': exclude,
        'fail_ci_if_error': fail_ci_if_error,
        'files': files,
        'flags': flags,
        'force': force,
        'git_service': git_service,
        'gcov_args': gcov_args,
        'gcov_executable': gcov_executable,
        'gcov_ignore': gcov_ignore,
        'gcov_include': gcov_include,
        'handle_no_reports_found': handle_no_reports_found,
        'job_code': job_code,
        'name': codecov_name,
        'network_filter': network_filter,
        'network_prefix': network_prefix,
        'os': validate_choice(
            'os',
            os,
            ['alpine', 'alpine-arm64', 'linux', 'linux-arm64', 'macos', 'windows'],
        ),
        'override_branch': override_branch,
        'override_build': override_build,
        'override_build_url': override_build_url,
        'override_commit': override_commit,
        'override_pr': override_pr,
        'plugins': plugins,
        'recurse_submodules': recurse_submodules,
        'report_code': report_code,
        'report_type': validate_choice(
            'report_type', report_type, ['test_results', 'coverage']
        ),
        'root_dir': root_dir,
        'run_command': validate_choice(
            'run_command',
            run_command,
            [
                'upload-coverage',
                'empty-upload',
                'pr-base-picking',
                'send-notifications',
            ],
        ),
        'skip_validation': skip_validation,
        'slug': slug,
        'swift_project': swift_project,
        'token': token,
        'url': url,
        'use_legacy_upload_endpoint': use_legacy_upload_endpoint,
        'use_oidc': use_oidc,
        'use_pypi': use_pypi,
        'verbose': verbose,
        'version': codecov_version,
        'working-directory': working_directory,
    }

    options = {key: value for key, value in options.items() if value is not None}

    if name is None:
        command = 'upload-coverage' if run_command is None else run_command
        name = f'Codecov Action ({command})'

    return action(
        name,
        'codecov/codecov-action',
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
