from setuptools import setup, find_packages

setup(
    name="api_models",
    version="0.1.1",
    packages=find_packages(where="."),  # Only include generated files
    package_dir={"": "."},
    install_requires=[
        "grpcio",
        "protobuf"
    ],
    include_package_data=True,
    python_requires=">=3.7",
)