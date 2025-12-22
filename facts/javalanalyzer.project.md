# Javalanalyzer

[repository](https://github.com/kyteware/javalanalyzer)

A Java package analyzer written all the way from scratch, down to the language processing.

I wrote this in the spring of 2025. The tool takes a path to a Java project and generates a package diagram.
I don't use anybody else's code for any of the language processing.
I wrote my own tokenizer using `regex` and generate my own ASTs. 
Although I only ended up rendering package diagrams, most of the infrastructure in the language processing is there to generate UML diagrams, etc.
The user interface is made using `swing`.

This was a fun way to learn how language compilers and interpreters read code.
I hope to do more stuff like this, projects like my own interpreters and compilers come to mind!
