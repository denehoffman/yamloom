from __future__ import annotations
from yamloom.actions.utils import validate_choice, check_string

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

__all__ = ['setup_mpi']


def setup_mpi(
    *,
    name: Ostrlike = None,
    version: str = 'v1',
    mpi: Ostrlike = None,
    args: Ostrlike = None,
    entrypoint: Ostrlike = None,
    condition: Oboolstr = None,
    id: Ostr = None,  # noqa: A002
    env: Mapping[str, StringLike] | None = None,
    continue_on_error: Oboollike = None,
    timeout_minutes: Ointlike = None,
) -> Step:
    options: dict[str, object] = {
        'mpi': validate_choice('mpi', mpi, ['mpich', 'openmpi', 'intelmpi', 'msmpi']),
    }

    options = {key: value for key, value in options.items() if value is not None}

    mpi_names = {
        'mpich': 'MPICH',
        'openmpi': 'Open MPI',
        'intelmpi': 'Intel MPI',
        'msmpi': 'Microsoft MPI',
    }

    if name is None:
        mpi_str = check_string(options.get('mpi'))
        if mpi_str:
            name = f'Setup {mpi_names[mpi_str]}'
        else:
            name = 'Setup MPI'

    return action(
        name,
        'mpi4py/setup-mpi',
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
