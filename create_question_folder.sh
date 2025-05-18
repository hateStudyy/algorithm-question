#!/bin/bash

# 题目编号
question_number=$1
# 语言扩展名
extension=$2

# 创建题目文件夹
mkdir -p "pta-basic-$question_number"

# 创建代码文件
touch "pta-basic-$question_number/solution.$extension"

# 创建解题思路文件
touch "pta-basic-$question_number/solution_notes.md"

echo "Created folder and files for question $question_number."