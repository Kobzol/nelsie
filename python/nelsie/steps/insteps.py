from typing import TypeVar, Generic, Sequence
import re

StepDef = int | Sequence[int] | str


def to_steps(obj):
    if isinstance(obj, InSteps):
        return obj
    else:
        return InSteps([obj])


T = TypeVar("T")
S = TypeVar("S")


def _extend_values(values: list[T], n: int) -> list[T]:
    if len(values) >= n:
        return values
    result = values[:]
    while len(result) < n:
        result.append(values[-1])
    return result


class InSteps(Generic[T]):
    def __init__(
        self, values=Sequence[T] | dict[StepDef, T], n_steps: int | None = None
    ):
        if isinstance(values, Sequence):
            if len(values) == 0:
                raise ValueError("Parameter 'values' cannot be an empty list")
            self.values = values
            self.n_steps = n_steps or len(values)
        elif isinstance(values, dict):
            self.values, n = self._values_from_dict(values)
            self.n_steps = n_steps or n
        else:
            raise ValueError("Invalid type for values")

    def get(self, step: int) -> T:
        if step < len(self.values):
            return self.values[step]
        else:
            return self.values[-1]

    @staticmethod
    def _values_from_dict(data):
        tmp = []
        n_steps = 1
        for key, value in data.items():
            in_steps = parse_steps(key)
            n_steps = max(n_steps, in_steps.n_steps)
            tmp.append((in_steps, value))

        used = [False] * n_steps
        values = [None] * n_steps

        for in_steps, value in tmp:
            for i in range(n_steps):
                if not in_steps.get(i):
                    continue
                if used[i]:
                    raise ValueError(f"Multiple definitions assigned for step {i+1}")
                used[i] = True
                values[i] = value
        if not all(used):
            raise ValueError(f"Value not defined for step {used.index(False) + 1}")
        return values, n_steps

    def map(self, fn):
        return InSteps([fn(v) for v in self.values], n_steps=self.n_steps)

    def zip(self, other: "InSteps[S]") -> "InSteps[(S, T)]":
        n = max(len(self.values), len(other.values))
        return InSteps(
            list(zip(_extend_values(self.values, n), _extend_values(other.values, n))),
            n_steps=max(self.n_steps, other.n_steps),
        )


def zip_in_steps(in_steps: Sequence[InSteps[T]]) -> InSteps[T]:
    assert in_steps
    n = max(len(s.values) for s in in_steps)
    values = [[s.get(i) for s in in_steps] for i in range(n)]
    n_steps = max(s.n_steps for s in in_steps)
    return InSteps(values, n_steps=n_steps)


def _expand_list(seq: Sequence, open: bool) -> InSteps[bool]:
    if not seq:
        return InSteps([False], 0)
    for value in seq:
        if not isinstance(value, int):
            raise ValueError("Step definition by sequence has to contains integers")
        if value < 1:
            raise ValueError("Step cannot be a zero or negative integer")
    max_value = max(seq)
    result = [False] * (max_value + (1 if not open else 0))
    for value in seq:
        result[value - 1] = True
    return InSteps(result, max_value)


def _expand_single(position: int, open: bool) -> InSteps[bool]:
    result = [False] * (position + (1 if not open else 0))
    result[position - 1] = True
    return InSteps(result, position)


STEP_DEF_CHECK_REGEXP = re.compile(
    r"^\s*\d+(?:\s*-\s*\d+)?(?:\s*,\s*\d+(?:\s*-\s*\d+)?)*\+?\s*$"
)
STEP_DEF_SPLIT_REGEXP = re.compile(r"\d+-\d+|\d+")


def parse_steps(obj: StepDef) -> InSteps[bool]:
    if isinstance(obj, bool):
        return InSteps([obj])

    if isinstance(obj, int):
        if obj < 1:
            raise ValueError("Step cannot be a zero or negative integer")
        return _expand_single(obj, False)

    if isinstance(obj, str):
        if not STEP_DEF_CHECK_REGEXP.match(obj):
            raise ValueError("Invalid step format")
        ranges = STEP_DEF_SPLIT_REGEXP.findall(obj)
        result = []
        for item in ranges:
            if "-" in item:
                start, end = map(int, item.split("-"))
                result.extend(range(start, end + 1))
            else:
                result.append(int(item))
        return _expand_list(result, "+" in obj)
    if isinstance(obj, Sequence):
        return _expand_list(obj, False)
    raise ValueError("Step cannot be a non-positive integer")