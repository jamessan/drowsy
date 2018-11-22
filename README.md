Drowsy provides a means to define delay [inhibitor locks] for sleep/shutdown events.

A YAML config file enumerates the actions to run and which events should run them.

```yaml
sleep:
    - exec: xscreensaver-command -lock
    - write: &weechat
        data: disconnect -all
        path: ~/.weechat/weechat_fifo
        append: true

shutdown:
    - write: *weechat
```

[inhibitor locks]: https://www.freedesktop.org/wiki/Software/systemd/inhibit/
