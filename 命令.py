#!/usr/bin/env python3
import os

# 创建专门的项目结构
dirs = [
    "github-action-builder",
    "github-action-builder/src",
    "github-action-builder/.github/workflows",
    "github-action-builder/assets"
]

files = [
    "github-action-builder/Cargo.toml",
    "github-action-builder/src/lib.rs",
    "github-action-builder/src/main.rs", 
    "github-action-builder/README.md",
    "github-action-builder/.gitignore"
]

for d in dirs:
    os.makedirs(d, exist_ok=True)
for f in files:
    open(f, 'w').close()

print("创建了专门的项目结构")