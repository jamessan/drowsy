// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © 2018 James McCoy <jamessan@jamessan.com>
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
enum InhibitorAction {
    #[serde(rename = "exec")]
    Exec(String),
    #[serde(rename = "write")]
    Write {
        data: String,
        path: std::path::PathBuf,
        #[serde(default)]
        append: bool,
    },
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Inhibitor {
    #[serde(default)]
    sleep: Vec<InhibitorAction>,
    #[serde(default)]
    shutdown: Vec<InhibitorAction>,
}

fn main() {
    let yml = r"
    sleep:
        - exec: xscreensaver-command -lock
        - write: &weechat
            data: disconnect -all
            path: ~/.weechat/weechat_fifo
    ";

    let inhibitor: Inhibitor = serde_yaml::from_str(&yml).unwrap();
    println!("{:?}", inhibitor);
}
