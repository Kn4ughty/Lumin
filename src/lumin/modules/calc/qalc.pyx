
from libcpp.string cimport string

cdef class PyCalculator:
    cdef Calculator* calc

    def __cinit__(self):
        self.calc = new Calculator()

    def __dealloc__(self):
        del self.calc

    def load_exchange_rates(self):
        self.calc.loadExchangeRates()

    def load_global_definitions(self):
        self.calc.loadGlobalDefinitions()

    def load_local_definitions(self):
        self.calc.loadLocalDefinitions()

    def calculate(self, expression: str, timeout: int = 2000) -> str:
        cdef MathStructure result 
        cdef string expression_cpp = expression.encode("utf-8")
        result = self.calc.calculate(expression_cpp)
        result.format()
        return result.print().decode("utf-8")
