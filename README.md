# 📝 notecli

A blazing-fast command-line note manager built in Rust for students and developers.

## Category — Text Editing

notecli falls under the **Text Editing** category because it provides 
a new and specialized way to create, edit, search and manage text notes 
directly from the command line. Instead of opening a graphical app to 
write text, notecli lets you interact with your notes using simple 
terminal commands — making text editing faster and more efficient for 
developers and students who already live in the terminal.

## The Problem

As a Health Informatics student I constantly juggle lecture notes, exam 
dates, code snippets and ideas across dozens of files and apps. 
Switching between apps breaks focus and wastes time. I needed to type 
one line in my terminal and have my thought saved immediately — without 
leaving my workflow.

## How notecli is Different from Existing Solutions

| Feature | notecli | Notion | Evernote | Apple Notes |
|---|---|---|---|---|
| Works in terminal | ✅ | ❌ | ❌ | ❌ |
| No account needed | ✅ | ❌ | ❌ | ❌ |
| Works offline | ✅ | ❌ | ❌ | ✅ |
| Single binary | ✅ | ❌ | ❌ | ❌ |
| Plain text storage | ✅ | ❌ | ❌ | ❌ |
| Free forever | ✅ | ❌ | ❌ | ✅ |
| scriptable/pipeable | ✅ | ❌ | ❌ | ❌ |
| No internet needed | ✅ | ❌ | ❌ | ✅ |

### Key differences explained:

**1. Terminal first**
Every existing note app requires you to leave your terminal and open 
a graphical interface. notecli stays inside your workflow — you never 
leave the terminal.

**2. No account, no cloud, no tracking**
Notion, Evernote and Google Keep require you to create an account and 
store your notes on their servers. notecli stores everything locally 
in a plain JSON file at ~/.notecli/notes.json — your notes belong to 
you forever.

**3. Single static binary**
notecli is one file. Download it, make it executable, run it. No 
installation wizard, no App Store, no dependencies, no runtime needed.

**4. Scriptable**
Because notecli is a CLI tool it can be combined with other Unix tools.
For example you could pipe output to grep, use it in shell scripts or 
automate note creation from other programs.

**5. Built for speed**
notecli is written in Rust — it starts instantly and uses almost zero 
memory. Notion and Evernote take seconds to load. notecli takes 
milliseconds.

## Features
- Add notes instantly from the terminal
- Full-text search across titles, bodies and tags
- Tag organisation by course or project
- Plain JSON storage in ~/.notecli/notes.json
- Single static binary — no runtime needed
- Offline first — notes never leave your machine

## Installation
Download the binary from the Releases page:
```bash
wget https://github.com/addoransford020-cmd/notecli/releases/latest/download/notecli-aarch64-unknown-linux-musl
chmod +x notecli-aarch64-unknown-linux-musl
sudo mv notecli-aarch64-unknown-linux-musl /usr/local/bin/notecli
```

## Usage
```bash
notecli add --title "My note" --body "Hello world" --tags demo
notecli list
notecli search demo
notecli view 0
notecli edit 0 --title "Updated title"
notecli delete 0
```

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
