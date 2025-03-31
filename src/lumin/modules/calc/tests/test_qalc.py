import sys
import os

# Add the src/lumin/modules/calc directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../..")))

from qalc import PyCalculator  # noqa: E402

calc = PyCalculator()
calc.load_exchange_rates()
calc.load_global_definitions()
calc.load_local_definitions()


def test_qalc():
    assert calc.calculate("2 + 2", 2000) == "4"
