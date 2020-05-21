# gitretro

## Features

- Read git history for one year ago same day as today
- Prettify the commits messages and send them to slack
- Install it as a osx user launch agent and it will run every weekday

## Usage

**Tell the app where to read git history and the slack channel to send the message:**

```sh
gitretro config
```

**Install as a user launch agent in `~/Library/LaunchAgents/`:**

```sh
gitretro installd
```

It runs every week day at 10:00 am

**Read history and send a message to slack:**

```sh
gitretro
```

## About

It's a mini project that I did for myself, because I want to learn rust more. It can be fun to remember my big team about their achievements a year ago. It's interesting to see how much the project has modified in one year, to see which changes still exist and which not.
