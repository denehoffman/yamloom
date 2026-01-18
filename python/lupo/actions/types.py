from __future__ import annotations

from typing import TypeAlias

from ..expressions import BooleanExpression, NumberExpression, StringExpression

Ostr: TypeAlias = str | None
Obool: TypeAlias = bool | None
Oint: TypeAlias = int | None
StringLike: TypeAlias = str | StringExpression
BoolLike: TypeAlias = bool | BooleanExpression
IntLike: TypeAlias = int | NumberExpression
Ostrlike: TypeAlias = StringLike | None
Oboolstr: TypeAlias = BooleanExpression | str | None
Oboollike: TypeAlias = BoolLike | None
Ointlike: TypeAlias = IntLike | None
StringOrBoolLike: TypeAlias = StringLike | BoolLike

__all__ = [
    "Ostr",
    "Obool",
    "Oint",
    "StringLike",
    "BoolLike",
    "IntLike",
    "Ostrlike",
    "Oboolstr",
    "Oboollike",
    "Ointlike",
    "StringOrBoolLike",
]
