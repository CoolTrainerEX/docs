# Server

World of Warcraft server.

## Installation

Install [Tailscale](https://tailscale.com/).

## Usage

### Initialization

```sh
git clone https://github.com/azerothcore/azerothcore-wotlk.git --depth 1 # or
git clone https://github.com/mod-playerbots/azerothcore-wotlk.git --branch=Playerbot # if need Playerbots
```

#### Dependencies

Clone repos into `modules` directory.

[Dependencies](deps.json)

#### Create

- [docker-compose.override.yml](create/docker-compose.override.yml)

### Run

```sh
docker compose up -d
```

To access console, run:

```sh
docker attach ac-worldserver
```
