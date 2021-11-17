# MySQL

## Information

We use the `mysql:8` image in this project.

What if you want to install MySQL client for command line, we recommend using `mysql-client`  \
and you can install it via `homebrew` if your OS is MacOS. Follow these steps below.

- Step1:
    - execute `brew install mysql-client` to install it.
- Step2:
    - execute `ln -s /usr/local/Cellar/mysql-client/<version>/bin/mysql /usr/local/bin/mysql` to create a soft link.

For more information about mysql image, see the [dockerhub page](https://hub.docker.com/_/mysql).