from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="rusty_core",
    version="0.1.0",
    rust_extensions=[RustExtension("rusty_core", binding="pyo3")],
    packages=["python_agent"],
    zip_safe=False,
)
