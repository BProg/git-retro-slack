# gitretro

## Features

- Read commits already in master
- Read branches which are not in master
- Combine commits and branches into a nice slack message
- Install it as a osx user launch agent and it will run every 2 weeks on monday

## How to use

1. clone this repo
2. build with feature production `cargo build --release --features "production"`
3. copy the executable from `./target/release/` anywhere and run it

## About

It's a mini project that I did for myself, because I want to learn the rust language more

## Story

[Read here some story on how it started](https://bprog.github.io/rust_slack_bot/)
