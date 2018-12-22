# note-rs
[![Crates.io](https://img.shields.io/crates/v/pf.svg?style=plastic)](http://crates.io/crates/note-rs)
[![Build Status](https://travis-ci.org/robatipoor/note-rs.svg?branch=master)](https://travis-ci.org/robatipoor/note-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/fqepq90uodd1muq6?svg=true)](https://ci.appveyor.com/project/robatipoor/note-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

note in command line


**install**

```sh
cargo install note-rs
```

**Build and install**

```sh
# build and install note-rs 
git clone https://github.com/robatipoor/note-rs \
&& cd note-rs \
&& make 
```

**Build dependency**

git, rustc, cargo, gnu make, binutils, upx

**run**

```sh
# write note 
$ note-rs hello 
# read all note 
$ note-rs 
# read line 1 
$ note-rs -r 1
# read line 1 until the 10
$ note-rs -r 1..10
# delete note line 2
$ note-rs -d 2 
```
