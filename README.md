Advent of Code 2017
===================

My (incomplete) solutions to 2022's Advent of Code, written in Rust. I found the time to do 9 of the 25 days, which is a pretty good rate for me.


## How to build and run

`cargo run --bin dayXX -- <input file>`


## Solutions I'm patting myself on the back for

For day 7 I almost went down the road of representing the file system in some kind of tree data structure. But luckily I realized that I could get away with making a sorted list of all file paths and simply adding it up starting from the bottom.

Using the `Ordering` enums returned by `cmp()` for covering the 9 possible directions of movement on day 9 was a neat idea, I think.
