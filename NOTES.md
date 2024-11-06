## Notes for wendall911
Currently lan-mouse does not have a client/server mode. I'm going to fork for
now so we can do this locally. Until encryption is in place, this will need to suffice.

1. # dnf install libadwaita-devel libXtst-devel libX11-devel
1. $ cargo build --release
1. $ ./target/release/lan-mouse -f gtk # test
1. $ cp target/release/lan-mouse path/to/wendall911-salt-states/files
1. Deploy with salt
1. Ensure lan-mouse service is enabled on systems needing to use it:
    1. $ systemctl --user daemon-reload
    1. $ systemctl --user enable --now lan-mouse.service
