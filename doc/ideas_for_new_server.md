Ideas for a new implementation of the server
============================================

An Unix-like server
-------------------

If we split the server software into multiple small programs, we can have them in different programming languages. This would allow us to use the most
convenient language for a given problem. Programs would call each other in order to accomplish complicated tasks.

Programs could pass information to each other via environment variables. For example the login program could set variables indicating the 
user, it's name as well as the type of terminal they are using.

After the login one could have the standard "browser" which asks the user for a page number and searches for that page in the "database" 
(probably just flat files on the disk, but could also be an actual database) and display it, handling follow-up pages. If the user selects a page that
cannot be handled by it, it'll call an external program set up for that page.

The browser, or a dedicated program, could handle forms. It would first display the form page, then allow the user to fill in the fields defined
in the metadata of that page. After submission of the input, the fields would be stored in environment variables.

Environment variables should have simple name spaces in order to not allow forms to change the user.

This could look something like this:
```
VTX_AUTH_USER Usernumber of the user
VTX_AUTH_NAME Name of the user
VTX_AUTH_CEPT1 Type of the terminal (for example autodetected during login)
VTX_USER_FIELD_COMMENT Some field from a form
```

When calling a new program, the default is to just hand over control of the file descriptors of the connection so the new program can talk directly
to the terminal via stdout and stdin. Alternatively a special mode selected for some programs could allow it to send a page structure to the browser
which would then handle the page as well as followup pages.


Potential Page Database
-----------------------
The pages could be stored in plain files in a directory structure. This makes it easy to handle them.
There could be subdirectories. Each subdirectory could have a file dir.pos indicating where it should be in the global structure.
The actual pages could be stored in files named `<pagenumber><subpage>.*`.
The pagenumber could be stored relatively to the prefix defined in the dir.pos file.

An example directory:
``` 
demosite/
demosite/dir.pos: Contains "123"
demosite/a.*: This page will be displayed when the user selects "*123#"
demosite/1a.*: This page will be displayed when the user selects "*1231#"
demosite/1b.*: This page will be displayeed when the user selects "*1231#" and then presses the # key (follow up page)
demosite/2.* :If the subpage suffix is missing, use this as the a page. Do no expect any follow up pages
```

Other files store the page content or relevant meta data. In case a file is missing, the browser will make a sensible decision to still be able to display it. (e.g. by skipping that data or only providing navigation back)
```
.cept: Actual data to be sent to the terminal.
.decdef: Decoder Definitionen, to be sent to the terminal first.
.selection: User selections, simple translation from 1-2 digit input to relative or absolute page number.
.meta: Metadata derived from FTZ 157 D2 Bulk
```
A selection file could look like this:
```
0=*123#  //Absolute page
1=2 //relative page withing directory
2=1b //Subpages could be allowed
```
A meta file could be like this:
```
source=file //Could be program for pages created by a program
billboard=true //Billboard page
```
