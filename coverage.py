from lupo import (  # noqa: INP001
    Events,
    Job,
    PullRequestEvent,
    PushEvent,
    Workflow,
    WorkflowCallEvent,
    WorkflowDispatchEvent,
    WorkflowSecret,
    action,
    script,
)
from lupo.actions.github.artifacts import download_artifact, upload_artifact
from lupo.actions.github.scm import checkout
from lupo.actions.toolchains.python import setup_uv
from lupo.actions.toolchains.rust import install_rust_tool, setup_rust
from lupo.expressions import context

print(
    Workflow(
        name='Coverage',
        jobs={
            'coverage-rust': Job(
                [
                    checkout(),
                    script(
                        'Install MPICH', 'sudo apt install -y clang mpich libmpich-dev'
                    ),
                    setup_rust(toolchain='nightly'),
                    install_rust_tool(tool=['just', 'cargo-llvm-cov']),
                    script('Generate Rust code coverage', 'just coverage-rust'),
                    upload_artifact(
                        path='coverage-rust.lcov', artifact_name='coverage-rust'
                    ),
                ],
                runs_on='ubuntu-latest',
                env={'CARGO_TERM_COLOR': 'always'},
            ),
            'coverage-python': Job(
                [
                    checkout(),
                    setup_uv(),
                    setup_rust(toolchain='stable'),
                    install_rust_tool(tool=['just']),
                    script('Generate Python code coverage', 'just coverage-python'),
                    upload_artifact(
                        path='coverage-python.xml', artifact_name='coverage-python'
                    ),
                ],
                runs_on='ubuntu-latest',
            ),
            'upload-coverage': Job(
                [
                    checkout(),
                    download_artifact(merge_multiple=True),
                    action(
                        'Upload coverage reports to Codecov',
                        'codecov/codecov-action',
                        ref='v5',
                        with_opts={
                            'token': context.secrets['CODECOV_TOKEN'],
                            'files': 'coverage-rust.lcov,coverage-python.xml',
                            'fail_ci_if_error': True,
                            'verbose': True,
                            'root_dir': context.github.workspace,
                        },
                    ),
                ],
                runs_on='ubuntu-latest',
                needs=['coverage-rust', 'coverage-python'],
            ),
        },
        on=Events(
            pull_request=PullRequestEvent(
                paths=['**.rs', '**.py', '.github/workflows/coverage.yml']
            ),
            push=PushEvent(
                branches=['main'],
                paths=['**.rs', '**.py', '.github/workflows/coverage.yml'],
            ),
            workflow_call=WorkflowCallEvent(
                secrets={'codecov_token': WorkflowSecret(required=True)}
            ),
            workflow_dispatch=WorkflowDispatchEvent(),
        ),
    )
)
