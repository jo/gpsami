Magellan
========

Magellan is a small GUI application to download data from a GPS
loggers and save it as GPX or KML.

It is written in Rust and uses Gtk3 for the UI and gpsbabel for the download part.

Requires libudev for listing devices, therefor require some effort to
run on non-Linux. Patches welcome.

See doc/devices.md for information about device support.

To build
--------

Once you have Rust installed, just do:

````
$ cargo build
````

Contributors
------------

Written and maintained by:

Hubert Figuiere <hub@figuiere.net>

Contributors:
