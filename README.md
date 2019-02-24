
# VNote

A command-line tool for taking micro notes.

## Usage

```console
foo@bar:~$ vnote add csharp "virtual methods result in dynamic dispatch"
```

## Todo

### Minimum Viable Product

- [x] Command to add a note
- [x] Save notes to file in user directory (the default book)
- [x] Search notes using regular expressions

### Future Features

- [ ] Cleanup `dead_code` annotations
- [ ] Implement using notebooks other than default `vnote` notebook
- [ ] Travis build and host releases on Github
- [ ] Coloured terminal output
- [ ] Linux version
- [ ] Organise search results according to relevance
- [ ] Sync notebooks with remote storage
- [ ] Relate notes to form graph
- [ ] Investigate lock files (how to implement proper synchronisation for file access)

### Nice to haves

- [ ] Full text search. Index file, concordance, strip stop words, fuzzy matching