# Freewrite on 2024-09-09

## Speed

Fasterface is a marketplace for commands. Users can find & run commands that match their specific needs, paying for each execution. Developers can sell their code to users directly, without worrying about user interface or payments.

Example:

* Alice wants to convert her photos from HEIC to JPG format.
* Alice says "convert my photos to JPG".
* Fasterface translates speech to text & runs a query over available commands.
* Fasterface finds a `convert-to-jpg` command developed by Bob.
* Fasterface displays the `convert-to-jpg` command to Alice & asks for confirmation.
* Alice confirms the command.
* Fasterface runs `convert-to-jpg` in the current directory.
* `convert-to-jpg` converts the photos to JPG.

This examples raises questions:

* How does the command know the directory to operate on?
* What if the directory contains images in other formats, but Alice wants to convert only HEIC to JPG?
* What if Alice wants to create a new directory with photos in JPG rather than 
