# Contributing to NodePM

First, thank you! Even if you are only looking at those document means you are interested in how the project is put together. I will try to make this as clear as possible. If it seems too detailed, or not like other contributing docs you may have seen, that's intentional.

## Assumed knowledge

So many docs these days expect you to be fairly versed in the area for whitch the program was written. While that may be true for a lot of people, I personally find it to be a barrier to getting involved in something you are interested in. If you are troubleshooting anything, it should be bugs in the program, not bugs or installing it, or setting up your workspace.

### * [Software development concepts](https://www.ibm.com/topics/software-development)
I'm not planning on you being an expert, but if you have never wondering how programs are made, or have wondered, but don't know the answer, you are in luck! There are tons of free resources online. Please let me know if you are having trouble locating some.

### * [Version Control](https://git-scm.com/) (git)

If you are interested in contributing to this project, you will need to understand version control. We use `git`, which is linked above. 

### More subject to come as they are surfaced as pain points

## Prerequisites

### General

* A computer

  if you are reading this on your phone, I recomend you trying to program there. While you _can_, it's going to be a very unenjoyable experence.

* A [GitHub account](https://github.com/signup) (optional)

  Note that while it is optional, you will need an account if you want to be able to add your changes to the main project. Git can be super confusing, but the basics are easy to grasp, Don't intimidated. As long as your try to rebase, you should not encounter many issues.

* An [Intregrated Development Enviroment](https://www.codecademy.com/article/what-is-an-ide) (ide) (optional)
  If you are a seasoned developer, I think I just heard a gasp. How can an IDE be optional, you ask? Just because you are using Vim/Emacs to write your own Notepad clones, doesn't mean a new developer needs to be overwhemed when they start learning.

  While an IDE or text editor will help you program, (and a good one will get out of your way and become invisible), you can [edit right on GitHub](https://github.com/github/dev) by pressing the perion (`.`) key on almost every page with code. Give it a try!

### Project

* We are using [Rust](https://www.rust-lang.org/)

* A good attitude. Please review the [code of conduct](https://github.com/drazisil/nodepm/blob/main/CODE_OF_CONDUCT.md). Your "right to free speech" does not exempt you from our "right to have a safe and inclusive space". If you really feel the need to wonder if something isn't allowed, it probably isn't. 

## Building

 * Clone the project (you did [learn git](#assumed-knowledge), didn't you?)
 * run `cargo build`
 * Have fun!