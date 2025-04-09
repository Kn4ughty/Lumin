
from setuptools import setup
from Cython.Build import cythonize
from setuptools.extension import Extension

qalc_extensions = [
    Extension(
        "qalc",
        ["src/lumin/modules/calc/qalc.pyx"],
        include_dirs=["/usr/include/libqalculate", "/usr/include/c++/14"],
        library_dirs=["/usr/lib", "/usr/local/lib"],
        libraries=["qalculate"],
        extra_compile_args=["-std=c++11"],
        language="c++"
    ),
]

setup(
    name="qalc",
    ext_modules=cythonize(qalc_extensions),
)


setup(
    name="sort",
    ext_modules=cythonize(Extension(
        "sort",
        ["src/lumin/models/sort.pyx"]
    ))
)
