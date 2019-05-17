# rust-arrow-sql
A POC of Arrow &lt;-> SQL conversion

## Goals

* To create a common interface between various databases and Apache Arrow, using the Rust implementation.
* To eventually read and write using databases' binary protocols (where available).
* To provide the ability to push down common analytics predicates such as filters, sorts, type conversions, and more.

## In-Scope Databases

1. PostgreSQL
2. MySQL
3. MSSQL

## Expected/Potential Uses

This is a POC to see if such work is feasible (spoiler, it probably is). Users of Apache Arrow could use this, and Dataframe implementations could use this to provide SQL data source/sink support.
