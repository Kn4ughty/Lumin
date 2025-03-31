
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

    # def calculate(self, expression: str, timeout: int = 2000) -> str:
    #     cdef string result = self.calc.calculateAndPrint(expression, timeout)
    #     return result.encode("utf-8")
    def calculate(self, expression: str, timeout: int = 2000) -> str:
        cdef string result 
        cdef string expression_cpp = expression.encode("utf-8")
        result = self.calc.calculateAndPrint(expression_cpp, timeout)
        return result.decode("utf-8")

