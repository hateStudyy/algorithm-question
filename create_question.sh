#!/bin/bash

# 题目编号
question_name=$1

# 进入文件夹
cd ./pta-basic-100

# 创建代码文件
touch "$question_name.java"

echo "Created file for question $question_name."