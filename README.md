---
title: Shawarma DB
author: Kevin Sullivan
creation date: 2023-09-28
last updated: 2023-09-28
version: 0.1.0
---

# Shawarma DB

This is, at the moment, a toy project for me to learn Rust.
I figured building a database might be one of the best exercises for learning a programming language.
It has all of the key features of interacting with a file system, creating a CLI, and even (hopefully) an HTTP webserver.
Depending on how it goes, I would also like to create a UI, either browser based or desktop, but still unsure the language. 

+ [x] create database
+ [ ] create tables, columns, rows, data types
    + [x] Design decision: keep opening database file or store in memory and pass around?
        + Quick reading it is best to keep file open to avoid overhead of accessing file contents. 
+ [ ] connect to the database to make updated
+ [ ] delete things
+ [ ] update things
+ [ ] read things
+ [ ] Small test suite for unit test of operations. 
+ [ ] Interactive terminal - for parsing Shawarma-Lang?

## The Journey

This is a learning experience for me. As I got to learn Python, Django, and React by building small task tracking application, I intend to do the same with Rust by building this database. 

### Creating the Database

Basically, I am going to have to get friendly with the [std::fs](https://doc.rust-lang.org/std/fs/index.html) module in Rust. 

I used the [std::fs::create_dir_all()](https://doc.rust-lang.org/std/fs/fn.create_dir.html) function to create the directory. for the user's database. However, this method takes in a [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html) struct as its parameter. One thing to note is difference between compile and run times. You use the `&` when creating a `$Path` because with the string literals the size is not known at compile time. So we just create a reference. 

I am also noticing the [std::io::Result](https://doc.rust-lang.org/std/io/type.Result.html) type that is used across IO operations and avoids wiring `io::Error`. 

The [File struct](https://doc.rust-lang.org/std/fs/struct.File.html) has many methods
