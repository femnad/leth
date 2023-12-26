# leth

A naive [urlview] replacement for opening multiple URLs with [mutt]. Stands on the shoulders of [skim].

## How to Use

After enabling in mutt (see below) run by hitting the corresponding keybinding, enable multiple selection with `<Tab>`.

## How to Enable in mutt

Something like the following in your `muttrc` should do the trick, which binds to `<Ctrl>+b`, same as `urlview`:

```
macro index,pager \cb "<pipe-message> leth<Enter>" "call leth to extract URLs out of a message"
macro attach,compose \cb "<pipe-entry> leth<Enter>" "call leth to extract URLs out of a message"
```

## Better Alternatives

[urlscan] is overall a more capable URL selector.

[mutt]: http://www.mutt.org/
[urlscan]: https://github.com/firecat53/urlscan
[urlview]: https://github.com/sigpipe/urlview
[skim]: https://github.com/lotabout/skim
