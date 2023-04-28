import pytest

from pydantic_core import core_schema

from ..conftest import Err, PyAndJson


@pytest.mark.parametrize(
    'input_value,expected',
    [
        ('jonas', 'Jonas'),
        ('jonas', 'jonas'),
        ('cool', 'Jonas'),
        (
            'dumb',
            Err(
                'Input should be a valid jonas, '
                "Unable to interpret input [type=jonas_parsing, input_value='dumb', input_type=str]"
            ),
        ),
    ],
)
def test_jonas_ok(py_and_json: PyAndJson, input_value, expected):
    v = py_and_json(core_schema.jonas_schema())
    jonas = v.validate_test(input_value)
    assert jonas == expected
