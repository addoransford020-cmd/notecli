# 📝 notecli

A blazing-fast command-line note manager built in Rust for students and developers.

## The Problem
As a Health Informatics student I constantly juggle lecture notes, exam dates, 
and ideas across dozens of files and apps. Switching between apps breaks focus. 
I needed to type one line in my terminal and have my thought saved immediately.

## Features
- Add notes instantly from the terminal
- Full-text search across titles, bodies and tags
- Tag organisation by course or project
- Plain JSON storage in ~/.notecli/notes.json
- Single static binary — no runtime needed
- Offline first — notes never leave your machine

## Usage
notecli add --title "My note" --body "Hello world" --tags demo
notecli list
notecli search demo
notecli view 0
notecli edit 0 --title "Updated title"
notecli delete 0

## How we attract users
- GitHub Pages website with live demo
- Dev.to blog post about building it
- Post in student developer communities on Reddit and Discord
- Open source — free forever

## Business Model
- Community tier — free and open source forever
- Pro tier coming soon — 4 euros per month for sync and web vault

## License
MIT
