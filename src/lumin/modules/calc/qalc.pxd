from libcpp.string cimport string

cdef extern from "libqalculate/qalculate.h":
    cdef cppclass Calculator:
        Calculator()  # Constructor
        void loadExchangeRates()
        void loadGlobalDefinitions()
        void loadLocalDefinitions()
        string calculateAndPrint(const string& expression, int timeout)
        MathStructure calculate(const string& expression)

    ctypedef enum IntervalDisplay:
        INTERVAL_DISPLAY_SIGNIFICANT_DIGITS,
        INTERVAL_DISPLAY_INTERVAL,
        INTERVAL_DISPLAY_PLUSMINUS,
        INTERVAL_DISPLAY_MIDPOINT,
        INTERVAL_DISPLAY_LOWER,
        INTERVAL_DISPLAY_UPPER,
        INTERVAL_DISPLAY_CONCISE,
        INTERVAL_DISPLAY_RELATIVE

    cdef struct PrintOptions:
        PrintOptions()
        # enum IntervalDisplay IntervalDisplay = INTERVAL_DISPLAY_PLUSMINUS
        IntervalDisplay interval_display
        # This is different from docs.
        # In my header file it says bool, in docs it says int
        bint use_unicode_signs

    cdef cppclass MathStructure:
        MathStructure()  # Constructor
        double toDouble()
        void format(const PrintOptions)
        string print(const PrintOptions)

    cdef cppclass EvaluationOptions:
        EvaluationOptions()

