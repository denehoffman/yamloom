from lupo import (  # noqa: INP001
    Events,
    Job,
    PullRequestEvent,
    PushEvent,
    Workflow,
    WorkflowDispatchEvent,
    action,
    script,
)
from lupo.actions.github.scm import checkout
from lupo.actions.toolchains.rust import install_rust_tool, setup_rust
from lupo.expressions import context

print(
    Workflow(
        name='CodSpeed Benchmarks',
        jobs={
            'benchmarks': Job(
                [
                    checkout(),
                    script(
                        'Install OpenMPI',
                        'sudo apt install -y openmpi-bin openmpi-doc libopenmpi-dev',
                    ),
                    setup_rust(toolchain='stable'),
                    install_rust_tool(tool=['cargo-codspeed']),
                    script(
                        'Build benchmark targets',
                        'cargo codspeed build --profile dist-release',
                    ),
                    action(
                        name='Run benchmarks',
                        action='CodSpeedHQ/action',
                        ref='v4',
                        with_opts={
                            'mode': 'simulation',
                            'run': 'cargo codspeed run',
                            'token': context.secrets['CODSPEED_TOKEN'],
                        },
                    ),
                ]
            )
        },
        on=Events(
            push=PushEvent(branches=['main']),
            pull_request=PullRequestEvent(),
            workflow_dispatch=WorkflowDispatchEvent(),
        ),
    )
)
