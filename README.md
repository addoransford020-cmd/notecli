# 📝 notecli — A Note Manager for the Terminal

## What is notecli?
notecli is a simple tool that lets you save, search and manage 
notes by typing one command in your terminal. No apps to open, 
no accounts to create, no internet needed.

## Which Category Does This Belong To?
notecli belongs to the **Text Editing** category because it 
provides a new and specialized way to write and manage text 
directly from the command line. It is not just a viewer — 
you can create, edit, search and delete text notes entirely 
from the terminal without ever opening a graphical application.

---

## Question 1: How does notecli compare to OTHER command-line note apps?

Most people compare notecli to graphical apps like Notion or 
Evernote. But there are also command-line note tools. Here is 
how notecli compares to them directly:

| Feature | notecli | jrnl | nb | todo.txt |
|---|---|---|---|---|
| Works without Python or Ruby | ✅ | ❌ | ❌ | ✅ |
| Single file — no installation | ✅ | ❌ | ❌ | ❌ |
| Simple — under 10 commands | ✅ | ✅ | ❌ | ✅ |
| Full title and body per note | ✅ | ✅ | ✅ | ❌ |
| Tag support | ✅ | ✅ | ✅ | ✅ |
| Full-text search | ✅ | ✅ | ✅ | ❌ |
| Built in Rust — instant startup | ✅ | ❌ | ❌ | ❌ |
| Plain JSON storage | ✅ | ❌ | ❌ | ✅ |

### Why notecli beats other CLI tools:

**vs jrnl**
jrnl requires Python and pip to install. Many students and 
developers do not want to manage Python environments just to 
take a note. notecli is one single binary — download it and 
it works. No Python, no pip, no virtual environments.

**vs nb**
nb is extremely powerful but has over 100 subcommands. It 
requires bash, git, curl and other tools. For a student who 
just wants to save a quick note it is far too complex. 
notecli has 6 commands and nothing else to install.

**vs todo.txt**
todo.txt only supports one line per entry — it is designed 
for tasks not notes. notecli supports a full title, a full 
body of text and multiple tags — much better for writing 
lecture notes, study summaries and ideas.

---

## Question 2: How does the 4 euro/month Pro tier work without an account?

This is a great question. Here is exactly how it works:

Traditional apps like Notion require you to create an account 
with an email and password. Your data is stored on their 
servers tied to your identity.

notecli Pro works differently using a KEY-BASED sync system:

**Step 1:** You pay 4 euros via a simple payment link.
No account is created. You receive a unique private sync key 
that looks like this: notecli-sync-a3f9b2c1d4e5

**Step 2:** On your first device you run:
notecli sync --key notecli-sync-a3f9b2c1d4e5

Your notes.json file is encrypted on your device using your 
key and uploaded to our sync server.

**Step 3:** On your second device you run the same command 
with the same key. Your encrypted notes are downloaded and 
decrypted locally.

**Result:** Your notes sync across devices. We never see 
your notes — only the encrypted file. No email. No password. 
No account. Just a key.

This is the same approach used by tools like Tailscale and 
Syncthing which millions of developers already trust.

If you lose your key your notes are still safe locally. 
You just cannot sync until you contact support with your 
payment receipt to get a new key.

---

## Question 3: Why should users care about "Single binary" and "Plain text storage"?

You are right that these sound like technical features. Here 
is what they mean for real users in plain language:

### Single binary — what the USER gets:
Imagine you are a student on a university computer. You 
cannot install software because you do not have admin rights.
With notecli you just download one file, make it executable 
and it runs. No installation wizard. No admin password. 
No IT department needed.

Or imagine you are helping a classmate set up a note tool. 
With jrnl you spend 20 minutes installing Python and pip. 
With notecli you spend 30 seconds downloading one file.

The user benefit is: **zero friction to get started.**

### Plain text storage — what the USER gets:
Imagine you used Evernote for 3 years and they shut down or 
change their pricing. Your notes are locked inside their 
system and hard to export.

With notecli your notes are in a plain JSON file on your 
computer. You can open it in any text editor. You can copy 
it to a USB stick. You can import it into any other tool. 
You own your data forever.

The user benefit is: **you are never locked in and never 
lose your notes.**

---

## Question 4: What is the difference between "works offline" and "no internet needed"?

You are absolutely right — these mean the same thing. 
We made a mistake by listing them separately. 

We have removed the duplicate. The correct single statement is:

**"Works offline"** — notecli stores all notes locally on 
your computer. You can add, search and read notes anywhere 
— on a plane, in a basement, in a hospital with no wifi. 
Your notes are always available because they never depend 
on a server or internet connection.

---

## Question 5: Is "pipeable" just a subfeature of "works on the command-line"?

Yes — you are correct. Pipeable is not a separate feature. 
It is simply what every good CLI tool does by default. 
We have removed it as a standalone feature.

The correct way to say it is: because notecli is a CLI tool 
it naturally works with other Unix tools. For example:

notecli list | grep exam
notecli search rust >> study-notes.txt

This is not a unique feature of notecli — it is a benefit 
of being a command-line tool in general.

---

## Features
- Add notes instantly from the terminal
- Full-text search across titles, bodies and tags
- Tag organisation by course or project
- Works completely offline — notes stored locally
- Zero friction setup — one binary, nothing else needed
- Your data is yours forever — plain JSON you can read anywhere

## How to Install

wget https://github.com/addoransford020-cmd/notecli/releases/download/v0.3.0/aarch64-unknown-linux-musl.tar.gz
tar -xzvf aarch64-unknown-linux-musl.tar.gz
sudo mv notecli /usr/local/bin/notecli
notecli --version

Verify it is statically linked:
file /usr/local/bin/notecli
Output: ELF 64-bit LSB executable, ARM aarch64, statically linked

## How to Use
notecli add --title "My note" --body "Hello world" --tags demo
notecli list
notecli search demo
notecli view 0
notecli edit 0 --title "Updated title"
notecli delete 0

## Customer Acquisition Strategy

### Who are our target users?
- Students who work in the terminal every day
- Linux developers who want a simple note tool
- Privacy-focused users who do not want cloud note apps

### How we find and attract them:

**1. GitHub organic discovery**
People searching for CLI note tools on GitHub will find us.
A clear README and good documentation brings users in naturally.
Open source means developers trust and share it freely.

**2. GitHub Pages website**
A simple landing page explains the tool clearly with examples.
Download instructions make it easy to get started immediately.

**3. Dev.to blog post**
Write about building notecli as a student using Rust.
Explain the problem it solves and link to the GitHub repo.
Developer blog posts drive consistent organic traffic.

**4. Developer communities**
Share in Discord servers for Linux and Rust developers.
Post in student developer groups and university forums.

**5. Word of mouth**
Students share useful tools with classmates naturally.
Free tier means zero barrier to trying it and sharing it.

### How we convert free users to Pro
Free tier builds trust — users love the tool first.
When they get a second device they want sync.
Pro tier at 4 euros per month solves exactly that problem.

## Pricing Strategy

### Free tier — forever
All 6 commands, unlimited notes, all platforms.
No credit card, no account, no time limit.
Goal: build a large base of happy users who trust the tool.

### Pro tier — 4 euros per month
Adds encrypted sync across devices using a private key.
No account needed — pay once via a payment link, get a key.
Goal: convert power users who work on multiple devices.

### Why 4 euros?
- Low enough that students can afford it
- High enough to cover sync server costs
- Cheaper than Notion Pro at 8 euros per month
- No annual commitment — cancel any time

## License
MIT — free to use for anything
