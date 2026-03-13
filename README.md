# Description

The main problem that this project is trying to solve is organizing media, such as pictures and videos by their metadata.
I have found myself running into this annoyance many times, and you yourself have probably faced it before.

I was trying to backup media from my phone onto external hard drive. I have noticed that all the media is scattered across
the disk on Android system. Eventually, I have created a bash script with exiftool that compares the dates and organizes media
throughout directories by their year and month of creation.

This is a kind of thing that I never knew I needed before as it organized thousands of files across file system in seconds.
I have seen the power and usability of this script and I have decided to make a CLI tool out of it. Bash is primitive
for this kind of file operations. I want to ensure there is no data loss ever and errors should be visible and understandable.
So I have decided to pickup Rust for this project.

## Ideas for later

It's important to note that I am trying to maintain core idea of this project for as long as possible. I will also try to make
core as versitile, straight forward and extensible as possible. This will allow the application to grow in features naturally
without major rewrites of code base. It's inspired by Emacs and Neovim architecture.

This project might evolve into something that organizes files for other programming projects, compare and edit metadata and
maybe, it can be useful for machine learning purposes. Everything eventually needs some sort of organization that can be
automated.

# Contributions

If you want to contribute to this project please write me an email. I will be happy to hear more opinions and ideas as well get
some help on actual code.

There are few rules that I want to acent:

1.  Please don't vibe code. Let this be human project.
2.  I will use slightly modified Linux version of K&R code formatting for this project.
    Rules will be out after first core release.
3.  Don't add anything that's not needed. This is lean and understandable project.

That's pretty much it for the beginning. Thank you.

