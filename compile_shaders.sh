#!/bin/bash

glslc ./shaders/triangle.frag -o ./shaders/triangle.frag.spv
glslc ./shaders/triangle.vert -o ./shaders/triangle.vert.spv

glslc ./shaders/colored_triangle.frag -o ./shaders/colored_triangle.frag.spv
glslc ./shaders/colored_triangle.vert -o ./shaders/colored_triangle.vert.spv
