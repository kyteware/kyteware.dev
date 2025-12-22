# Optimal Keyboard Layout

[repository](https://github.com/kyteware/eloquent)

This was a super fun project, I saw a bunch of people trying this on YouTube and I wanted to give it my own spin. 

There was one thing that set mine apart from most other ones I saw:
I directly tested peoples' finger speeds and incorporated them into my simulation. My process for the final result was:

- Making a live tool to test people's finger movement speeds (using `Rust` and the `Bevy` game engine)
- Compile the raw data into average speeds of individual fingers going to different keys.
- Scrape the text from the top 100 Wikipedia articles.
- Make a simulation to see how long it would take to type those articles using an arbitrary keyboard layout.
- Use simulated annealing to effectively choose and test millions of keyboard layouts to find the one with the best performance. In the end, this is what I got:

```
o d g m f n h u l e 
 a s t b p y k i r ; 
  z x c v w q j , . /
```