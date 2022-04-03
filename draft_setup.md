# General information
There are several main components interacting in this project. We will be building our design off the ROS framework () which is language agnostic (can use anything). Which is the reason why we are building our project off of Rust!

### Why Rust? ###

**A few main benefits:**
- Statically typed (unlike Python)
- Memory safety out of the box (no null pointers, dangling references)
- Thread safety out of the box
- Package management (which is non-existent with C++)

However, there are also a few pratical reasons outside the tech specs of the language. We have to face the reality that we do not know everything. We are not well versed in the intricacies of the frameworks, programming languages, and even the concepts we are trying to implement. And it may seem counter-intuitive that having everyone learn a whole new language is time efficient, but in reality we are saving ourselves from a world of pain by using Rust (over the alternative of C++). We would like to increase our chances of bowling a perfect 300, and Rust is basically the gutterball guard rail preventing us from messing up. 

I do concede that the documentation is less available for Rust based ROS projects, however there is enough to build off of for our purpose. We are going to build off of the package `rosrust` (which is just a Rust client implementation for ROS).

However I highly recommend that we take inspiration from the `openrr` on how to architecture our code since it has many many functions that we need for our purposes already implemented. Unfortunately, after some testing, not all of the code compiles so we can't build off of it directly (not to mention a lack of documentation).
