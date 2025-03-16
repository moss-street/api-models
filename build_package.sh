#!/bin/bash

set -e  # Exit on error

echo "📂 Creating package directory..."
mkdir -p api_models

echo "🔨 Generating Python Protobuf files..."
python -m grpc_tools.protoc -Iprotos --python_out=api_models --grpc_python_out=api_models protos/*.proto

echo "📦 Creating __init__.py for package..."
echo 'from .common_pb2 import *' > api_models/__init__.py
echo 'from .common_pb2_grpc import *' >> api_models/__init__.py
echo 'from .users_pb2 import *' >> api_models/__init__.py
echo 'from .users_pb2_grpc import *' >> api_models/__init__.py

echo "📦 Building Python package..."
python setup.py sdist bdist_wheel

echo "✅ Build complete!"