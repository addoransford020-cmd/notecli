# notecli — Instant Structured Notes for the Terminal

## The Problem

You are deep in your terminal working on code or studying.
A thought hits you. You need to write it down immediately.

Every existing solution breaks your flow:

Cloud apps like Notion and Evernote need a browser,
an account and internet — switching breaks your focus.

Desktop apps like Obsidian and Onyx are powerful but
heavy — they need installation and a graphical interface.

Other CLI tools like jrnl need Python. nb needs bash,
git and curl configured. Joplin CLI needs Node.js.
tnote and tdo need Python or Go installed first.

A plain text file has no structure — finding notes
later means reading through everything manually.

notecli solves all of this with one command:

notecli add --title "Idea" --body "Cache the response" --tags backend,urgent

Structured. Tagged. Searchable. Done in 2 seconds.

---

## The Unique Position of notecli

After comparing every major note taking tool available,
notecli occupies a position no other tool fills:

THE ONLY CLI NOTE TOOL THAT IS:
- Structured — every note has title, body and tags
- Zero dependencies — no Python, Ruby, Node or Go needed
- Single static binary — download and run immediately
- Built in Rust — starts in milliseconds, never crashes

This combination does not exist in any other tool.

---

## Broad Comparison — Where notecli Fits

CLOUD NOTE APPS:
Notion — powerful but needs account, browser and internet
Evernote — needs account and subscription for full features
Google Keep — simple but tied to Google account
Microsoft OneNote — free but needs Microsoft account
Simplenote — minimal but requires account and internet

DESKTOP NOTE APPS:
Obsidian — excellent but needs app installation and GUI
Onyx — beautiful typography but macOS only, needs app
Standard Notes — encrypted but needs account and app
Capacities — modern but cloud based and needs account

CLI NOTE TOOLS:
jrnl — needs Python and pip installed
nb — needs bash, git, curl — over 100 commands, complex
Joplin CLI — needs Node.js installed, complex setup
tnote — needs Python installed
tdo — opinionated, needs Go installed
todo.txt — no body field, tasks only, no structure
idea — lightweight but no tag system or search

TUI NOTE TOOLS:
clin — text interface inside terminal, needs installation
VB Notes — local search but GUI based

notecli compared to all of them:

| Feature | notecli | jrnl | nb | Obsidian | Notion |
|---|---|---|---|---|---|
| Works in terminal | YES | YES | YES | NO | NO |
| Zero dependencies | YES | NO | NO | NO | NO |
| Single binary | YES | NO | NO | NO | NO |
| No account needed | YES | YES | YES | YES | NO |
| Works offline | YES | YES | YES | YES | NO |
| Structured notes | YES | Partial | YES | YES | YES |
| Instant setup | YES | NO | NO | NO | NO |
| Built in Rust | YES | NO | NO | NO | NO |
| Free forever | YES | YES | YES | NO | NO |

The gap notecli fills:
Every tool that is structured needs setup or an account.
Every tool with zero setup has no structure.
notecli is the only one with BOTH.

---

## What Category Does notecli Belong To?

notecli belongs to the TEXT EDITING category.

It provides a completely new interaction model for
creating and editing text. Instead of opening an app,
clicking buttons and navigating menus, you edit text
through simple structured commands directly in your
terminal. The focus is on making text capture and
retrieval faster and more efficient than any existing
approach.

---

## What Makes notecli Different From Each Competitor

vs Notion and Evernote:
No account. No internet. No browser. Works offline.
One command versus 5 clicks.

vs Obsidian:
No app to install. No graphical interface needed.
Works on any Linux system with no setup at all.

vs Onyx:
Onyx focuses on typographic beauty — displaying notes
like a typeset book on macOS. notecli focuses on the
opposite: speed of capture on any Linux terminal.
Different tools for completely different workflows.

vs jrnl:
jrnl requires Python and pip. notecli needs nothing.
Download one file and it works. No runtime needed.

vs nb:
nb has over 100 commands and requires bash, git and
curl configured. notecli has 6 commands. That is all.

vs Joplin CLI:
Joplin CLI needs Node.js installed and a sync account
for full features. notecli works standalone with zero
dependencies and no account ever.

vs todo.txt:
todo.txt supports only one line per entry — built for
tasks not notes. notecli has title, body and tags —
built for real notes and study summaries.

vs a plain text file:
A text file has no structure. Finding something later
means reading everything. notecli gives you instant
search by keyword or tag across all your notes.

---

## What "Structured Notes" Actually Means

Every note in notecli has three fields:
- Title — a short name for the note
- Body — the full content
- Tags — labels to organise by topic or priority

This is not an AST or programming language concept.
It simply means every note is organised the same way
so you can always find it later instantly.

A plain text file is searchable with grep but has no
structure. notecli gives you structure with the same
simplicity as a text file.

---

## Static Linking Verified

Verify the binary yourself:

wget https://github.com/addoransford020-cmd/notecli/releases/download/v0.3.0/aarch64-unknown-linux-musl.tar.gz
tar -xzvf aarch64-unknown-linux-musl.tar.gz
file notecli

Output confirms:
notecli: ELF 64-bit LSB executable, ARM aarch64, statically linked

---

## How to Install

wget https://github.com/addoransford020-cmd/notecli/releases/download/v0.3.0/aarch64-unknown-linux-musl.tar.gz
tar -xzvf aarch64-unknown-linux-musl.tar.gz
sudo mv notecli /usr/local/bin/notecli
notecli --version

## How to Use

notecli add --title "My note" --body "Hello world" --tags demo
notecli list
notecli search demo
notecli view 0
notecli edit 0 --title "Updated title"
notecli delete 0

---

## Customer Acquisition Strategy

Who are our target users:
- Students who work in the terminal every day
- Linux and backend developers
- Privacy focused users avoiding cloud note apps
- Developers on shared university computers with no admin rights

How we find and attract them:

1. GitHub organic discovery
People searching for CLI note tools on GitHub find us.
Good README and documentation brings users in naturally.
Open source means developers trust and share it freely.

2. GitHub Pages website
Simple landing page with clear examples and download links.
Shows exactly what problem notecli solves and who it is for.

3. Dev.to blog post
Explain the problem notecli solves as a student using Rust.
Link back to GitHub repo to drive consistent organic traffic.

4. Developer communities
Share in Discord servers for Linux and Rust developers.
Post in student developer groups and university forums.

5. Word of mouth
Students share useful tools with classmates naturally.
Free tier means zero barrier to trying and sharing it.

How we convert free users to Pro:
Free tier builds trust — users love the tool first.
When they get a second device they want sync.
Pro tier at 4 euros per month solves exactly that.

---

## Pricing Strategy

Free tier — forever:
All commands, unlimited notes, all platforms.
No credit card. No account. No time limit.
Goal: build a large base of happy users who trust the tool.

Pro tier — 4 euros per month:
Adds encrypted sync across devices using a private key.
No account needed — pay once and receive a sync key.
Run notecli sync --key YOUR_KEY on each device.
Notes are encrypted on your device before uploading.
We never see your notes — only the encrypted file.
Goal: convert power users working on multiple devices.

Why 4 euros:
- Affordable for students
- Covers sync server running costs
- Cheaper than Notion Pro at 8 euros per month
- No annual commitment — cancel any time

---

## License
MIT — free to use for anything
