gpsami
======

gpsami is a small GUI application to download data from a GPS loggers
and save it as GPX or KML.

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

License
-------

This software is licensed under the GNU Public License v3. See COPYING.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.


Contributors
------------

Written and maintained by:

Hubert Figuière <hub@figuiere.net>

Contributors:
