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

+ [ ] create database
+ [ ] create tables, columns, rows, data types
+ [ ] connect to the database to make updated
+ [ ] delete things
+ [ ] update things
+ [ ] read things
+ [ ] Small test suite for unit test of operations. 

## The Journey

This is a learning experience for me. As I got to learn Python, Django, and React by building small task tracking application, I intend to do the same with Rust by building this database. 

### Creating the Database

Basically, I am going to have to get friendly with the [std::fs](https://doc.rust-lang.org/std/fs/index.html) module in Rust. 

I used the [std::fs::create_dir_all()](https://doc.rust-lang.org/std/fs/fn.create_dir.html) function to create the directory. for the user's database.

The [File struct](https://doc.rust-lang.org/std/fs/struct.File.html) has many methods