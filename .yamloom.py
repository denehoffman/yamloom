from dataclasses import dataclass
from yamloom.actions.github.artifacts import upload_artifact, download_artifact
from yamloom.actions.packaging.python import maturin
from yamloom.actions.toolchains.python import setup_python, setup_uv
from yamloom.actions.github.scm import checkout
from yamloom.expressions import context
from yamloom import (
    Workflow,
    Events,
    PushEvent,
    PullRequestEvent,
    WorkflowDispatchEvent,
    Permissions,
    Job,
    Matrix,
    Strategy,
    script,
    Environment,
)


@dataclass
class Target:
    runner: str
    target: str


def create_build_job(
    job_name: str, name: str, targets: list[Target], python_versions: list[str]
) -> Job:
    return Job(
        [
            checkout(),
            setup_python(
                python_version='\n'.join(python_versions),
                architecture=context.matrix.platform.python_arch.as_str()
                if name == 'windows'
                else None,
            ),
            maturin(
                name='Build wheels',
                target=context.matrix.platform.target.as_str(),
                args=f'--release --out dist --interpreter {" ".join(python_versions)}',
                sccache=~context.github.ref.startswith('refs/tags/'),
                manylinux='musllinux_1_2' if name == 'musllinux' else None,
            ),
            upload_artifact(
                path='dist',
                artifact_name=f'wheels-{name}-{context.matrix.platform.target}',
            ),
        ],
        runs_on=context.matrix.platform.runner.as_str(),
        strategy=Strategy(
            fast_fail=False,
            matrix=Matrix(
                platform=[
                    {
                        'runner': target.runner,
                        'target': target.target,
                        'python_arch': (
                            ('arm64' if target.target == 'aarch64' else target.target)
                            if name == 'windows'
                            else None
                        ),
                    }
                    for target in targets
                ],
            ),
        ),
    )


release_workflow = Workflow(
    name='Build and Release',
    on=Events(
        push=PushEvent(branches=['main'], tags=['*']),
        pull_request=PullRequestEvent(),
        workflow_dispatch=WorkflowDispatchEvent(),
    ),
    permissions=Permissions(contents='read'),
    jobs={
        'linux': create_build_job(
            'Build Linux Wheels',
            'linux',
            [
                Target('ubuntu-22.04', target)
                for target in [
                    'x86_64',
                    'x86',
                    'aarch64',
                    'armv7',
                    's390x',
                    'ppc64le',
                ]
            ],
            [
                '3.9',
                '3.10',
                '3.11',
                '3.12',
                '3.13',
                '3.13t',
                '3.14',
                '3.14t',
                'pypy3.11',
            ],
        ),
        'musllinux': create_build_job(
            'Build (musl) Linux Wheels',
            'musllinux',
            [
                Target('ubuntu-22.04', target)
                for target in [
                    'x86_64',
                    'x86',
                    'aarch64',
                    'armv7',
                ]
            ],
            [
                '3.9',
                '3.10',
                '3.11',
                '3.12',
                '3.13',
                '3.13t',
                '3.14',
                '3.14t',
                'pypy3.11',
            ],
        ),
        'windows': create_build_job(
            'Build Windows Wheels',
            'windows',
            [
                Target('windows-latest', 'x64'),
                Target('windows-latest', 'x86'),
                Target('windows-11-arm', 'aarch64'),
            ],
            [
                '3.9',
                '3.10',
                '3.11',
                '3.12',
                '3.13',
                '3.13t',
                '3.14',
                '3.14t',
                'pypy3.11',
            ],
        ),
        'macos': create_build_job(
            'Build macOS Wheels',
            'macos',
            [
                Target('macos-15-intel', 'x86_64'),
                Target('macos-latest', 'aarch64'),
            ],
            [
                '3.9',
                '3.10',
                '3.11',
                '3.12',
                '3.13',
                '3.13t',
                '3.14',
                '3.14t',
                'pypy3.11',
            ],
        ),
        'sdist': Job(
            [
                checkout(),
                maturin(name='Build sdist', command='sdist', args='--out dist'),
                upload_artifact(path='dist', artifact_name='wheels-sdist'),
            ],
            name='Build Source Distribution',
            runs_on='ubuntu-22.04',
        ),
        'release': Job(
            [
                download_artifact(),
                setup_uv(),
                script(
                    'Publish to PyPI',
                    'uv publish --trusted-publishing always wheels-*/*',
                ),
            ],
            name='Release',
            runs_on='ubuntu-22.04',
            condition=context.github.ref.startswith('refs/tags/')
            | (context.github.event_name == 'workflow_dispatch'),
            needs=['linux', 'musllinux', 'windows', 'macos', 'sdist'],
            permissions=Permissions(id_token='write', contents='write'),
            environment=Environment('pypi'),
        ),
    },
)


if __name__ == '__main__':
    release_workflow.dump('.github/workflows/release.yml')
