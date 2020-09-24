# gitretro

## Features

- Read git history for one year ago same day as today
- Prettify the commits messages and send them to slack
- Install it as a osx user launch agent and it will run every weekday

## Usage

```log
gitretro

Usage:
gitretro run            runs the program
gitretro installd       installs the launch agent parameters in user's space
gitretro config         configures the tool

Options:
--help          prints this message
```

## About

It's a mini project that I did for myself, because I want to learn rust more. It can be fun to remember my big team about their achievements a year ago. It's interesting to see how much the project has modified in one year, to see which changes still exist and which not.

## TODO

- [ ] integrate quote of the day API
- [ ] logging to a file while launch agent is running
- [x] optimize and reduce app size
- [x] improve commands with help messages for other commands
- [x] install daemon from cli
- [x] create a beautiful message summary with emojis
- [x] small cli that setups the config file on run
- [x] read from a configuration file (confy)
- [x] make a daemon (demonize)
- [x] send message to slack
- [x] read git history

