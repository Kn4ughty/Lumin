
# 2. file naming
Should end in the .desktop file extension

## 2.1 Desktop file id
Data dirs are stored in path `$XDG_DATA_DIRS`
to determine desktop file id, remove the applications/ prefix, and turn `/` into `-`
e.g `/usr/share/applications/foo/bar.desktop` has ID `foo-bar.desktop`

# 3 file format
Encoded in UTF-8

---
lines begging with `#` are comments and should be ignored

---
a group header is formatted like this `[groupheader]`
group names may contain all ASCII characters except `[]`
group names are unique
all data belongs to the group header until a new header is defined

There must be a group header named `[Desktop entry]`
There can be comments before this header

---
Entries in the file are key value pairs, in this format
`Key=Value`
Space before and after the `=` sign should be ignored. `=` is the delimiter
Only the characters `A-Za-z0-9` should be in key names (woah regex anyone??)
Case is signficant, (`Name` != `NAME`)

keys in the same group may not have the same name. Different groups yes

# 4 Possible value types
1. string
    - All ASCII except control characters 
2. localestring
    - User displayable, encoded in UTF-8
3. iconstring
    - Names of icons. May be absolute paths or symbolic names
    - See [[http://freedesktop.org/wiki/Standards/icon-theme-spec]]
4. boolean
    - One of these two strings, `true`, `false`
5. numeric
    - Valid floating point number recognised by %f for scanf in C
    - I hope python just works


The escape sequences \s, \n, \t, \r, and \\ are supported for values of type string, localestring and iconstring, meaning ASCII space, newline, tab, carriage return, and backslash, respectively. 

# 5 Localised values
I am ignoring this. This will be english only for now, until real users exist.

# 6 Recognised keys
I am not writing this all down read this:
[[https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html]]

# 7 The `Exec` Key
A command line consisting of an executable program with optional arguments
If there is no full path, look it up in $PATH.
I think for this i should just use the subprocess thing and treat it like a command
THen i dont need to worry about the path.

However Things can have special codes, for exmaple vlc has this:
`Exec=/usr/bin/vlc --started-from-file %U`
The %U means a list of URLS for it to open. 
This is intended for use with like file managers, 
and all the other field codes seem to be also intended for that usecase.

Current plan is to strip the command from this field codes, and run without.
However `%%` must be intepreted as `%` to get the literal `%`


# Example linux .desktop file for vlc
```
[Desktop Entry]
Version=1.0
Name=VLC media player
GenericName=Media player
Comment=Read, capture, broadcast your multimedia streams
Name[af]=VLC-mediaspeler
GenericName[af]=Mediaspeler
... 
 More names in different languages
...
Exec=/usr/bin/vlc --started-from-file %U
TryExec=/usr/bin/vlc
Icon=vlc
Terminal=false
Type=Application
``` 


