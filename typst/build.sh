#!/bin/bash

# Script to compile Turn-Formal Typst documents

echo "Compiling Turn-Formal documents with Typst..."

# Get current directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

# Compile the paper
echo "Compiling paper.typ..."
typst compile paper.typ

# Compile the presentation
echo "Compiling presentation.typ..."
typst compile presentation.typ

echo "Done! Generated files:"
echo "- paper.pdf"
echo "- presentation.pdf" 