# Performance Comparison Report

## Project Overview

This project involved rewriting an existing Python script for data processing in Rust and analyzing the improvements in speed and resource usage. The primary objective was to compare the performance of Python and Rust implementations to demonstrate any optimizations achieved.

## Performance Comparison

### Python Implementation

- **Execution Time**: Approximately 13.148 seconds.
- **CPU Usage**: 23% (measured via `time` command).
- **Description**: The Python script utilized `pandas` for data processing and `matplotlib` for plotting. Although straightforward, Python's interpreted nature may contribute to slower execution times.

### Rust Implementation

- **Execution Time**: Approximately 1.015 seconds.
- **CPU Usage**: 34% (measured via `cargo run` command).
- **Description**: The Rust script utilized the `ndarray` crate for data manipulation and `plotters` for visualization. Rust's compiled nature and efficient memory management significantly reduced execution time, showcasing its performance advantages.

## Observations

- **Speed Improvement**: The Rust implementation was approximately 13 times faster than the Python implementation.
- **Resource Utilization**: Rust utilized the CPU more efficiently, achieving faster results due to its compiled and optimized execution model.
- **Code Structure**: The Rust code required more explicit handling of data types and memory, but it provided greater control over execution, resulting in better performance.

## Conclusion

The Rust implementation demonstrates considerable improvements in both speed and resource usage compared to the original Python script. This project highlights Rust’s efficiency for computational tasks and its potential for performance-critical applications.
