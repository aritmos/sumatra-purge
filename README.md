# sumatra-cleanse
 A simple program to remove non-existing files from SumatraPDF's filestate cache.

## Problem:
- [SumatraPDF](https://www.sumatrapdfreader.org/) is an amazing lightweight FOSS PDF viewer for Windows, and my personal daily driver for the past 5 years.
- SumatraPDF takes note of all the PDFs that the user opens, and stores their state (page, zoom, bookmarks, etc) within `SumatraPDF-settings.txt`
- This cache is also used for the viewer's in built file finder (using the command palette):

![SumatraPDF-search](sumatrapdf-search.jpg)

- The problem is that this **file cache is never purged**. Every PDF that one opens with SumatraPDF gets saved to it, and these logs persist after the file has been deleted or moved, resulting in possibly hundreds of results polluting both the settings file and the command palette's results.

## Solution
- SumatraPDF is a simple program that loads up SumatraPDF's settings file, scans through the list of saved filestates, and only keeps the ones for which the file still exists in the stated location.

- In my case I found that around 40% of the logged files were no longer existent:
```
Permissions Size User    Name
.rw-r--r--   45k aritmos SumatraPDF-settings.txt
.rw-r--r--   28k aritmos SumatraPDF-settings-purged.txt
```
