#!/bin/bash

set -e  # Exit on error

PROTO_DIR="protos"
OUT_DIR="api_models"

echo "📂 Creating package directory..."
mkdir -p ${OUT_DIR}

echo "🔍 Finding all proto files..."
PROTO_FILES=$(find ${PROTO_DIR} -name "*.proto")

echo "🔨 Generating Python Protobuf files..."
python -m grpc_tools.protoc \
    --proto_path=${PROTO_DIR} \
    --python_out=${OUT_DIR} \
    --grpc_python_out=${OUT_DIR} \
    ${PROTO_FILES}

echo "📦 Ensuring package structure..."
for dir in $(find ${OUT_DIR} -type d); do
    touch "${dir}/__init__.py"
done

echo "📦 Building Python package..."
python setup.py sdist bdist_wheel

echo "✅ Build complete!"
