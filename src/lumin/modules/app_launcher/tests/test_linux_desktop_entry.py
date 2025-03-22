# I wish i could use relative imports
# Sadly it doesnt work
# from .. import linux_desktop_entry as lde
from lumin.modules.app_launcher import linux_desktop_entry as lde


def test_parse_exec_str():
    assert lde.parse_exec_string("a") == ["a"]
    assert lde.parse_exec_string("blender %f") == ["blender"]

    assert lde.parse_exec_string("\\\\") == ["\\"]
    assert lde.parse_exec_string("\\$") == ["$"]
    assert lde.parse_exec_string("\\&") == ["&"]
    assert lde.parse_exec_string("%%") == ["%"]
