# iak
A color-coded ls for clearer file display.

By redeveloping the ls command and adding color coding, file information can be displayed more clearly and intuitively.

# Background
When there are many files, it can be difficult to distinguish them at a glance.

# Description
This project redevelops the ls command to improve file readability. By adding color coding, it helps users identify file types more quickly and intuitively.

# Usage
Usage:
  iak [FILE]
  iak -l [OPTIONS] [FILE]

Argument:
  [FILE]  Path of the file or directory to display.
          If omitted, the current directory is shown.

Options:
  -l, --long              Show detailed file information

      --humanize          Display file sizes in a human-readable format
      --tagline           Show a summary of README.md
      --pdf-title         Show the title inside a PDF file
      --respect-ignore    Respect rules such as .gitignore
      --new-mark          Mark files created or updated within 24 hours as "new"
      --sort <key>        Specify the sort order
                          [name | size | mtime]

  -h, --help              Show help information

  -V, --version           Show version information
