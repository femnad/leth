# leth

A naive [urlview] replacement for opening multiple URLs with [mutt]. Stands on the shoulders of [skim].

## How to Use

After enabling in mutt (see below) run by hitting the corresponding keybinding, enable multiple selection with `<Tab>`.

## How to Enable in mutt

Something like the following in your `muttrc` should do the trick, which binds to `<Ctrl>+b`, same url `urlview`:

```
macro index,pager,attach,compose \cb "\
<enter-command> set my_pipe_decode=\$pipe_decode pipe_decode<Enter>\
<pipe-message> leth<Enter>\
<enter-command> set pipe_decode=\$my_pipe_decode; unset my_pipe_decode<Enter>" \
"call leth to extract URLs out of a message"
```

The above snippet nicked from the default global `muttrc` in an Ubuntu installation.

[urlview]: https://github.com/sigpipe/urlview
[mutt]: http://www.mutt.org/
[skim]: https://github.com/lotabout/skim
