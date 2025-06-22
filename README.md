# ebook-organiser
Organise ebooks in folders.

This is a tiny, simple tool to orgaise eBooks into files.
This only exists as all other tools I found were either too complex, or missing the simple case of just sorting files based on their metadata.
Currently it supports the following formats:
- epub
- mobi

## How to use

- First run the program with `ebook-organiser save-config` to save a default config file to your profile.
- Then edit this config to point to the folders where your ebooks are stored, and where your likely to ingest from.
- - Note: At runtime you can temporarily override the config by passing the `--config` argument.
- - Note: Also, you can override the source folder; so I suggest using a different folder for both (maybe Downloads/Exports for source?)
- Then run `ebook-organiser organise <optional source path>` to organise your ebooks.

* All movements of files require confirmation, so you can review the changes before they are made.
* If it's unsure on authors it will ask.
