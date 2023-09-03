# dlsite-asmr-player

## About

This is web application to browse and listen your dlsite ASMR library.

## Notice

This application is wip. Database content may lost in the future. Also, security
is inadequate. Do not publish this on public network.

Only Japanese is supported for now.

## Setup

1. Create container

Create `docker-compose.yml` like this.

```yaml
version: "3"
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: pass
      POSTGRES_USER: user
      POSTGRES_DB: db
    volumes:
      - postgres:/var/lib/postgresql/data
    command: postgres -c log_destination=stderr -c log_statement=all -c log_connections=on -c log_disconnections=on
    logging:
      options:
        max-size: "10k"
        max-file: "5"
  app:
    image: dap
    build: .
    depends_on:
      - db
    environment:
      DATABASE_URL: postgresql://user:pass@db:5432/db
    ports:
      - 14567:14567


volumes:
  postgres:
```

And start container.

```
docker compose up -d
```

2. Set voice folder

Open `http://[server_ip]:14567` and login. Default password is `password`. After
login, select `Settings` from drawer and set scan directory.

3. Scan

Select `Scan` menu and click `Start scan`. After scan finished, items are shown
in home page.

Folders whose name contain `RJ\d+` are scanned.
