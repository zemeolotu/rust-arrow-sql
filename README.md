# rust-arrow-sql
A POC of Arrow &lt;-> SQL conversion

## Goals

* To create a common interface between various databases and Apache Arrow, using the Rust implementation.
* To eventually read and write using databases' binary protocols (where available).
* To provide the ability to push down common analytics predicates such as filters, sorts, type conversions, and more.

## Non-goals

### Common Abstraction over Databases (JDBC-like)

There's a need for, and benefit in, a JDBC-like abstraction for Rust. Let's call it RDBC (Rust DataBase Connectivity). This project is not that, though I wish I could work on that too. 
There are a lot of common abstractions which we'll have to repeat in this project, such as converting a `Row` from each database into an Arrow record (or better, rows into a `RecordBatch`). I'm fine with this repetition for now.

### Async vs Sync

I'm not focusing on creating async vs sync interfaces, though my professional-gut-feel tells me that a sync interface should suffice. 

## In-Scope Databases

1. PostgreSQL
2. MySQL
3. MSSQL

## Expected/Potential Uses

This is a POC to see if such work is feasible (spoiler, it probably is). Users of Apache Arrow could use this, and Dataframe implementations could use this to provide SQL data source/sink support.

This project is only/mainly suitable for bulk data transfer, so can be used for ETL and data-analysis tasks.