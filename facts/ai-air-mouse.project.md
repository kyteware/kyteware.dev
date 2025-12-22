# AI Air Mouse

[repository](https://github.com/kyteware/ai-air-mouse)

Machine learning project to track hand motions and translate it to mouse movements!

I worked on this project as my capstone leaving high school.
This was originally intented to be a hardware project using a bunch of IMUs attached to a hand, but my parts were faulty and I had to change course.

I had some fantastic afternoons in the maker club experimenting 

[video](https://github.com/user-attachments/assets/a35d6bf7-c1aa-4ada-b963-75f7ccddd48d)

This project consists of:
- Using the mediapipe handtracking model to trace the user's hand position
- A customized hand gesture model trained on my own thousands of hand images to detect the clicking gesture
- Integration with Linux through `uinput` virtual devices to let the user directly control the cursor and click.
