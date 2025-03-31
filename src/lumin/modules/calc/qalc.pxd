from libcpp.string cimport string

cdef extern from "libqalculate/qalculate.h":
    cdef cppclass Calculator:
        Calculator()  # Constructor
        void loadExchangeRates()
        void loadGlobalDefinitions()
        void loadLocalDefinitions()
        string calculateAndPrint(const string& expression, int timeout)
        MathStructure calculate(const string& expression)

    cdef cppclass MathStructure:
        MathStructure()  # Constructor
        double toDouble()
        void format()
        string print()

    cdef cppclass EvaluationOptions:
        EvaluationOptions()

    cdef cppclass PrintOptions:
        PrintOptions()
