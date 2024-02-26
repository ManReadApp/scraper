# ManRead Scraper

Part of [ManRead](https://github.com/ManReadApp/ManRead)

Example:
```
image[href] div div#test.est2 ... img.cover
```

##
- field can contain letter, number or _
- [] is the value that will be extracted
- valid values are `href`, `text`, `html`, `src`, `attr=custom`,
- selector is required

## Selector details
### Prefixes
- '.' => class
- '#' => id
- no prefix => name

### More than one identifier
- ['.', '#', ' ', ' ... '] are separators between identifiers
- ' ' => next descendant
- ' ... ' => descendant with unknown htmltags between
- '.' => additional class on self
- '#' => additional id on self