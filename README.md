# BeaverTodos

![build status](https://github.com/jeremydumais/BeaverTodos/actions/workflows/rust.yml/badge.svg)

A Linux terminal todo list application written in Rust

Has been tested on Ubuntu 20.04 and ArchLinux

## How to install the latest binaries
```bash
curl -sSL https://raw.githubusercontent.com/jeremydumais/BeaverTodos/main/install.sh | sh
```

## How to build and install from source

Rust is needed to build BeaverTodos.

Follow these installation instructions https://www.rust-lang.org/tools/install

```bash
git clone https://github.com/jeremydumais/BeaverTodos.git
cd BeaverTodos
sh install.sh -b
```

## Features in version 0.2.2

- Remove a todo
- Edit a todo
- Purge the todo list
- Get the next todo to work on
- Fetch the details of a todo
- Add the usage when no command is supplied

## How it works

### Add a todo
```bash
beaver add This is a test
```

### Add a todo with a high priority
```bash
beaver add This is a test -p=h
```

### Print the todo list (by priority)
```bash
beaver list
```

### Get the todo to work on
```bash
beaver next
```

### Complete a todo
```bash
beaver done <id>
```

### Edit a todo (changing title and priority)
```bash
beaver edit <id> -t=Another title -p=m
```

## Usage
<pre>
beaver command [OPTIONS]

COMMANDS:
    -v, --version                  Print version info and exit
    -h, --help                     Prints help information
    list                           Display the todo list
    add                            Add a new todo
    edit                           Edit an existing todo
    done                           Complete a todo
    next                           Display the next todo to work on
    fetch                          Display the details of a specific todo
    remove                         Delete a todo
    purge                          Delete all completed todos

USAGE BY COMMAND:
    add title [OPTIONS]
        title                      The title (text) of the todo
        -p=x, --priority=x         The priority of the todo, possible values are H, M and L
                                   for High, Medium and Low
    edit id [OPTIONS]
        -t=x, --title=x            The title (text) of the todo
        -p=x, --priority=x         The priority of the todo, possible values are H, M and L
                                   for High, Medium and Low
    list [OPTIONS]
        -a, --all                  Display all todos even those who are completed
        -s=x, --sort=x             Sort the todo list by one of the following:
                                   prioritydesc: Sort by priority from High to Low (Default)
                                   priority: Sort by priority from Low to High
                                   creationtimedesc: Sort by creation time by more to less recent
                                   creationtime: Sort by creation time by less to more recent
    done id                        The id of the todo to complete
    next                           <No argument required>
    fetch id                       The id of the todo to display
    remove id                      The id of the todo to delete
    purge                          <No argument required>
</pre>
