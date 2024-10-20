# How to Use Workjournal
---
note: you are highly advised to alias these commands to streamline usage in the terminal! Especially before the interactive mode is added in the future, this will help usability out a lot.

## First Principles - A View from Cruising Altitude

Workjournal is designed to be used in jobs where you deal with many jobs, tickets, requests, etc over the course of the day, and no system already exists for keeping notes about those. In my situation personally, I am a machine programmer at a steel fab shop, and I have a lot of paperwork for a lot of different jobs going across my desk every day, both for programming and for quality control. I like to keep detailed notes on all of this, in case I need to reference something multiple days later. Up til now I've done it all by hand in Notepad, and used the PowerShell equivalent of `grep` to grab all the notes for a particular order if I ever needed to refer back to them. Workjournal is designed to handle this whole process, with a bunch more functionality coming in the future too, and everything is done in the command line for maximum speed and ergonomics of use.



## Commands

### `workjournal chactive <jobnumber>`

The change-active command changes the active job / order. Job numbers are stored as a u32 (at least for the time being) so alphanumeric values or numbers greater than 4,294,967,295 will cause problems. The active job is stored in the configuration file, and workjournal updates with value using regex find-and-replace in an effort to preserve comments in configuration. This behavior will be changed in the future and is not meant to be permanent.

### `workjournal mknote <note....>`

This command makes a note. All text typed after `mknote` is saved as a those, prepended with an HH:MM timestamp, and the active job number.

### `workjournal print <jobnumber>`

This command prints out all of the notes taken about a given job number. It uses the internal libraries developed for `ripgrep` to do this.

## Configuration

An example configuration file is provided in DOC/config.yaml.

### Location of the configuration file

Workjournal uses the `directories` crate to handle the locations of config directories in a platform agnostic way. Based on your system, here is where workjournal will expect to find the config.yaml file:

| Platform | Path |
| -------- | --------- |
| Windows | C:\Users\Alice\AppData\Local\workjournal\johnnybgoode\config\config.yaml |
| Linux | /home/Alice/.config/workjournal/config.yaml |
| MacOS | /Users/Alice/Library/Application Support/com.SwissArmyWrench.workjournal/config.yaml |

### `active_job`

This value is the current job number. It is overwritten by the system when the `chactive` command is issued. The program expects this to be a u32 - any values in this field that are not valid u32s will cause problems

### `logging_folder`

This is an absolute path to the folder where workjournal will save / search for notes. 

### `file_extension` [optional]

This is an optional value to specify a file extension to use on the files workjournal creates. For example, on Windows, it would be best to set this to ".txt" or perhaps ".md".
