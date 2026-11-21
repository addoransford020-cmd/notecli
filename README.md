# 📝 notecli — Notes That Work Like Code

> "If your workflow lives in the terminal, your notes should too."

## The Unique Idea Behind notecli

Most note apps are built for people who think in documents.
notecli is built for people who think in code.

Just like source code, your notes in notecli are:
- Tagged — like labels on a git issue
- Searchable — like grep on your codebase
- Structured — stored in plain JSON you can script
- Version-controllable — back up with git like any project
- Composable — pipe output into other terminal tools

This is the twist: notecli does not just store notes.
It makes your notes part of your development workflow.

A developer taking notes should feel the same as a developer
writing code — fast, structured and in full control.

## Which Category Does This Belong To?

notecli belongs to the Text Editing category.

notecli provides a completely new way to create and edit text.
Instead of opening a graphical editor, you type one command.
Instead of storing notes in a proprietary format, you get
plain JSON you can open, edit and move anywhere.
Instead of clicking through menus, you search and filter
notes the same way you search and filter code.

It is text editing reimagined for the terminal workflow.

Example — instead of this:
1. Open Notion in browser and wait 5 seconds
2. Click New Page
3. Type your note
4. Go back to terminal

You do this:
notecli add --title "Bug found" --body "Line 42 null pointer" --tags bug,urgent

Done in 2 seconds. Never left the terminal.

## How notecli Compares to Other CLI Note Apps

| Feature | notecli | jrnl | nb | todo.txt |
|---|---|---|---|---|
| Works without Python or Ruby | YES | NO | NO | YES |
| Single file no installation | YES | NO | NO | NO |
| Simple under 10 commands | YES | YES | NO | YES |
| Full title and body per note | YES | YES | YES | NO |
| Tag support | YES | YES | YES | YES |
| Full-text search | YES | YES | YES | NO |
| Built in Rust instant startup | YES | NO | NO | NO |
| Plain JSON scriptable storage | YES | NO | NO | NO |
| Notes feel like code | YES | NO | NO | NO |

vs jrnl
jrnl requires Python and pip to install. notecli is one
single binary. No Python, no pip, no virtual environments.

vs nb
nb has over 100 subcommands and requires bash, git and curl.
notecli has 6 commands and nothing else to install.

vs todo.txt
todo.txt only supports one line per entry built for tasks.
notecli supports a full title, body and tags — better for
lecture notes and study summaries.

vs Onyx
Onyx is a beautiful macOS app focused on typographic
brilliance — notes displayed like a typeset book.
notecli has the opposite focus: speed and structure
over beauty. For the developer who wants to capture
a thought in 2 seconds without leaving the terminal.

## What Makes notecli Special for Users

No setup needed — what this means for you:
Download one file and it works immediately. No admin rights
needed. Works on any university computer. Less time setting
up means more time taking notes.

Plain JSON storage — what this means for you:
Your notes are saved in a file you can open in any text
editor. Never locked into notecli. Copy to USB, back up
with git or import into any other tool. Your notes
belong to you forever.

Works offline — what this means for you:
Use notecli on a plane, in a basement, in a hospital lab
with no wifi. Notes always available because they live
on your computer not on a server.

## Answering the Professor Questions Directly

Why are single binary and plain JSON valid for users?
Single binary means a student on a university computer
with no admin rights can still use notecli.
Plain JSON means your notes belong to you forever.
If notecli disappears your notes are still readable
by any tool. You are never locked in.

What is the difference between works offline and no internet?
There is no difference — we listed it twice by mistake.
The correct statement is simply: works offline.

Is pipeable just a subfeature of CLI?
Yes — you are right. We removed it as a standalone feature.
It is simply what every good CLI tool does by default.

## Static Linking Verified

Download and verify yourself:
wget https://github.com/addoransford020-cmd/notecli/releases/download/v0.3.0/aarch64-unknown-linux-musl.tar.gz
tar -xzvf aarch64-unknown-linux-musl.tar.gz
file notecli

Expected output:
notecli: ELF 64-bit LSB executable, ARM aarch64, statically linked

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

## Customer Acquisition Strategy

Who are our target users:
- Students who work in the terminal every day
- Linux developers who want notes inside their workflow
- Privacy focused users who do not want cloud note apps

How we find and attract them:

1. GitHub organic discovery
People searching for CLI note tools on GitHub find us.
Clear README and documentation brings users in naturally.
Open source means developers trust and share it freely.

2. GitHub Pages website
Simple landing page explains the tool with live examples.
Download instructions make it easy to get started.

3. Dev.to blog post
Write about building notecli as a student using Rust.
Explain the problem it solves and link to the GitHub repo.
Developer blog posts drive consistent organic traffic.

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

## Pricing Strategy

Free tier — forever:
All 6 commands, unlimited notes, all platforms.
No credit card, no account, no time limit.
Goal: build a large base of happy users who trust the tool.

Pro tier — 4 euros per month:
Adds encrypted sync across devices using a private key.
No account needed — pay once, receive a sync key.
Goal: convert power users who work on multiple devices.

How the sync key works:
- Pay via a simple payment link — no account created
- You receive a unique private sync key
- Run notecli sync --key YOUR_KEY on each device
- Notes are encrypted on your device before uploading
- We never see your notes — only the encrypted file
- No email, no password — just your key

Why 4 euros:
- Affordable for students
- Covers sync server running costs
- Cheaper than Notion Pro at 8 euros per month
- No annual commitment — cancel any time

## License
MIT — free to use for anything
