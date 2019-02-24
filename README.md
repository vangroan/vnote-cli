
# VNote

A command-line tool for taking micro notes.

## Usage

Adding a note

```console
foo@bar:~$ vnote add csharp "virtual methods result in dynamic dispatch"
  # adding [csharp] virtual methods result in dynamic dispatch
  ✓ added 70fdcc14-6235-48d2-bc65-c6e98d23e5f5
```

Finding a note
```console
foo@bar:~$ vnote find "method"
  # searching...
  ✓ results found
  csharp
   - virtual methods result in dynamic dispatch
```

## Todo

### Minimum Viable Product

- [x] Command to add a note
- [x] Save notes to file in user directory (the default book)
- [x] Search notes using regular expressions

### Future Features

- [ ] Add topic argument(s) to `find` command
- [ ] Allow the `add` command to detect typos (Levenshtein distance?) and warn before creating a junk topic. Example:
```console
foo@bar:~$ vnote add jvascript "truthy values can be coerced to boolean with `!!`"
    ! did you mean to use topic "javascript" or create "jvascript"
[enter] use "javascript", [c] create "jvascript", [esc] abort:
```
- [ ] Cleanup `dead_code` annotations
- [ ] Implement using notebooks other than default `vnote` notebook
- [ ] Travis build and host releases on Github
- [x] Coloured terminal output
- [ ] Linux version
- [ ] Organise search results according to relevance
- [ ] Sync notebooks with remote storage
- [ ] Relate notes to form graph (what can we do with this, I wonder?)
- [ ] Investigate lock files (how to implement proper synchronisation for file access)

### Nice to haves

- [ ] Full text search. Index file, concordance, strip stop words, fuzzy matching