# PostgreSQL

## Information

We use the `postgres:13.4` image in this project.

What if you want to install PostgreSQL client for command line, we recommend using `libpq`  \
and you can install it via `homebrew` if your OS is MacOS. Follow these steps below.

- Step1:  \
  execute `brew install libpq` to install it.
- Step2:  \
  execute `ln -s /usr/local/Cellar/libpq/<version>/bin/mysql /usr/local/bin/psql` to create a soft link.

For more information, see the [dockerhub page](https://hub.docker.com/_/postgres).