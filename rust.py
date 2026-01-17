from lupo import (  # noqa: INP001
    Concurrency,
    Events,
    Job,
    PullRequestEvent,
    PushEvent,
    Workflow,
    WorkflowDispatchEvent,
    script,
)
from lupo.actions.github.scm import checkout
from lupo.actions.toolchains.python import setup_uv
from lupo.actions.toolchains.rust import install_rust_tool, setup_rust
from lupo.expressions import context

print(
    Workflow(
        jobs={
            'clippy': Job(
                [
                    checkout(),
                    script(
                        'Install OpenMPI',
                        'sudo apt update\nsudo apt install -y clang libopenmpi-dev',
                    ),
                    setup_rust(components=['clippy']),
                    install_rust_tool(tool=['just']),
                    script('Run Clippy', 'just clippy'),
                ],
                runs_on='ubuntu-latest',
            ),
            'build-check-test': Job(
                [
                    checkout(),
                    script(
                        'Install OpenMPI',
                        'sudo apt update\nsudo apt install -y clang libopenmpi-dev',
                    ),
                    setup_rust(components=['clippy']),
                    install_rust_tool(tool=['just']),
                    install_rust_tool(tool=['just', 'cargo-hack', 'nextest']),
                    setup_uv(),
                    script('Run cargo-hack check', 'just hack-check'),
                    script('Run tests', 'just test'),
                ],
                runs_on='ubuntu-latest',
                condition=(context.github.event_name == 'pull_request')
                | ~context.github.event['pull_request']['draft'].as_bool(),
            ),
        },
        on=Events(
            push=PushEvent(branches=['main'], tags=['*']),
            pull_request=PullRequestEvent(
                opened=True, reopened=True, synchronize=True, ready_for_review=True
            ),
            workflow_dispatch=WorkflowDispatchEvent(),
        ),
        env={'CARGO_TERM_COLOR': 'always'},
        concurrency=Concurrency(
            group=f'{context.github.workflow}-{context.github.event["pull_request"]["number"].as_bool() | context.github.ref.as_bool()}',
        ),
    )
)
