from lupo import (  # noqa: INP001
    Concurrency,
    Environment,
    Events,
    Job,
    Matrix,
    Permissions,
    PullRequestEvent,
    PushEvent,
    Step,
    Strategy,
    Workflow,
    WorkflowDispatchEvent,
    action,
    script,
)
from lupo.actions.github.artifacts import download_artifact, upload_artifact
from lupo.actions.github.scm import checkout
from lupo.actions.toolchains.python import setup_python, setup_uv
from lupo.expressions import context

cpu_platforms = {
    'linux': [
        {'runner': 'ubuntu', 'target': arch}
        for arch in ['x86_64', 'x86', 'aarch64', 'armv7', 's390x']
    ],
    'musllinux': [
        {'runner': 'ubuntu', 'target': arch}
        for arch in ['x86_64', 'x86', 'aarch64', 'armv7']
    ],
    'windows': [{'runner': 'windows-latest', 'target': 'x64'}],
    'macos': [
        {'runner': 'macos-15-intel', 'target': 'x86_64'},
        {'runner': 'macos-latest', 'target': 'aarch64'},
    ],
}
mpi_platforms = {
    'linux': [{'runner': 'ubuntu', 'target': 'x86_64'}],
    'windows': [{'runner': 'windows-latest', 'target': 'x64'}],
    'macos': [
        {'runner': 'macos-15-intel', 'target': 'x86_64'},
        {'runner': 'macos-latest', 'target': 'aarch64'},
    ],
}


def build_wheels(os: str, manifest_path: str, *, free_threaded: bool = False) -> Step:
    opts = {
        'target': context.matrix['platform']['target'].as_str(),
        'sccache': ~context.github.ref.starts_with('refs/tags/'),
    }
    if os == 'linux':
        opts['manylinux'] = 'auto'
    elif os == 'musllinux':
        opts['manylinux'] = 'musllinux_1_2'

    return action(
        f'Build{" free-threaded " if free_threaded else " "}wheels',
        'PyO3/maturin-action',
        ref='v1',
        with_opts=opts,
        args=f'--release --out dist --manifest-path {manifest_path}{" -i python3.13t" if free_threaded else ""}',
    )


def sdist(manifest_path: str) -> Step:
    return action(
        'Build sdist',
        'PyO3/maturin-action',
        ref='v1',
        with_opts={'command': 'sdist'},
        args=f'--out dist --manifest-path {manifest_path}',
    )


all_build_job_names = [
    *[f'cpu-{os}' for os in cpu_platforms],
    'cpu-sdist',
    *[f'mpi-{os}' for os in mpi_platforms],
    'mpi-sdist',
]

print(
    Workflow(
        jobs={
            **{
                f'cpu-{os}': Job(
                    [
                        checkout(),
                        setup_python(python_version='3.13'),
                        build_wheels(os, 'py-laddu-cpu'),
                        setup_python(python_version='3.13', freethreaded=True),
                        build_wheels(os, 'py-laddu-cpu', free_threaded=True),
                        upload_artifact(
                            path='dist',
                            artifact_name=f'cpu-{os}-{context.matrix["platform"]["target"]}',
                        ),
                    ],
                    condition=(context.github.event_name == 'pull_request')
                    | ~context.github.event['pull_request']['draft'].as_bool(),
                    runs_on=context.matrix['platform']['runner'].as_str(),
                    strategy=Strategy(matrix=Matrix(platform=platform)),
                )
                for os, platform in cpu_platforms.items()
            },
            'cpu-sdist': Job(
                [
                    checkout(),
                    sdist('py-laddu-cpu/Cargo.toml'),
                    upload_artifact(path='dist', artifact_name='cpu-sdist'),
                ],
                condition=(context.github.event_name == 'pull_request')
                | ~context.github.event['pull_request']['draft'].as_bool(),
                runs_on='ubuntu-latest',
            ),
            **{
                f'mpi-{os}': Job(
                    [
                        checkout(),
                        action('Setup MPI', 'mpi4py/setup-mpi', ref='v1'),
                        setup_python(python_version='3.13'),
                        build_wheels(os, 'py-laddu-mpi'),
                        setup_python(python_version='3.13', freethreaded=True),
                        build_wheels(os, 'py-laddu-mpi', free_threaded=True),
                        upload_artifact(
                            path='dist',
                            artifact_name=f'mpi-{os}-{context.matrix["platform"]["target"]}',
                        ),
                    ],
                    condition=(context.github.event_name == 'pull_request')
                    | ~context.github.event['pull_request']['draft'].as_bool(),
                    runs_on=context.matrix['platform']['runner'].as_str(),
                    strategy=Strategy(matrix=Matrix(platform=platform)),
                )
                for os, platform in cpu_platforms.items()
            },
            'mpi-sdist': Job(
                [
                    checkout(),
                    sdist('py-laddu-mpi/Cargo.toml'),
                    upload_artifact(path='dist', artifact_name='mpi-sdist'),
                ],
                condition=(context.github.event_name == 'pull_request')
                | ~context.github.event['pull_request']['draft'].as_bool(),
                runs_on='ubuntu-latest',
            ),
            'release': Job(
                [
                    checkout(),
                    download_artifact(),
                    setup_uv(),
                    script(
                        'Prepare wheels',
                        'set -euo pipefail',
                        'mkdir -p dist/cpu dist/mpi',
                        'cp cpu-*/* dist/cpu/',
                        'cp mpi-*/* dist/mpi/',
                    ),
                    script(
                        'Publish py-laddu-cpu to PyPI',
                        'uv publish --trusted-publishing always dist/cpu/*',
                    ),
                    script(
                        'Publish py-laddu-mpi to PyPI',
                        'uv publish --trusted-publishing always dist/mpi/*',
                    ),
                    script(
                        'Build py-laddu sdist',
                        'mkdir -p dist/laddu',
                        'uv build py-laddu --out-dir dist/laddu',
                    ),
                    script(
                        'Publish py-laddu to PyPI',
                        'uv publish --trusted-publishing always dist/laddu/*',
                    ),
                ],
                name='Release',
                runs_on='ubuntu-latest',
                needs=all_build_job_names,
                environment=Environment('pypi'),
                permissions=Permissions(id_token='write', contents='write'),  # noqa: S106
                condition=context.github.ref.starts_with('refs/tags/')
                | (context.github.event_name == 'workflow_dispatch'),
            ),
        },
        on=Events(
            push=PushEvent(branches=['main']),
            pull_request=PullRequestEvent(
                opened=True, reopened=True, ready_for_review=True, synchronize=True
            ),
            workflow_dispatch=WorkflowDispatchEvent(),
        ),
        concurrency=Concurrency(
            f'{context.github.workflow}-{context.github.event["pull_request"]["number"].as_bool() | context.github.ref.as_bool()}',
            cancel_in_progress=True,
        ),
        permissions=Permissions(contents='read'),
    )
)
