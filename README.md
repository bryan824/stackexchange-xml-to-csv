# stackoverflow-xml-to-csv

This is inspired by <https://github.com/SkobelevIgor/stackexchange-xml-converter> while I am preparing the dataset following this article [Working with Large Datasets: BigQuery (with dbt) vs. Spark vs. Dask](https://medium.com/@cjmcguicken/working-with-large-datasets-bigquery-with-dbt-vs-spark-vs-dask-92e596ce8e06).

For a personal usecase, it now only supports single file conversion.

```shell
stackoverflow-xml-to-csv input.xml output.csv
```

## What is good

- It is about 4~5 times faster than the go version(Using [academia.stackexchange.com.7z](https://archive.org/download/stackexchange/academia.stackexchange.com.7z) for benchmark)

  ```shell
  hyperfine "../target/release/stackoverflow-xml-to-csv PostHistory.xml PostHistory.csv"
  Benchmark 1: ../target/release/stackoverflow-xml-to-csv PostHistory.xml PostHistory.csv
  Time (mean ± σ):      1.745 s ±  0.079 s    [User: 1.317 s, System: 0.384 s]
  Range (min … max):    1.671 s …  1.933 s    10 runs
  ```

  ```shell
  hyperfine "./stackexchange-xml-converter -result-format=csv -source-path=/Users/bryan/src/ideas/lf_compute/stackexchange-xml-to-csv/xml/PostHistory.xml -store-to-dir=/Users/bryan/src/ideas/lf_compute/stackexchange-xml-to-csv/xml/csv"
  Benchmark 1: ./stackexchange-xml-converter -result-format=csv -source-path=/Users/bryan/src/ideas/lf_compute/stackexchange-xml-to-csv/xml/PostHistory.xml -store-to-dir=/Users/bryan/src/ideas/lf_compute/stackexchange-xml-to-csv/xml/csv
  Time (mean ± σ):      9.049 s ±  0.252 s    [User: 8.410 s, System: 0.854 s]
  Range (min … max):    8.679 s …  9.423 s    10 runs
  ```

- Making use of nix flake, it supports cross compilation from macOS to linux.
