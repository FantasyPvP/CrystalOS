# CrystalOS
the initial aim of this project was to follow a blog series on how to make a custom operating system found here:

https://os.phil-opp.com/

with the github repo for his project here:

https://github.com/phil-opp/blog_os

After reading and implementing the features from the final chapter, (async/await) I could find
no further instruction on how to continue with the project from there despite the author of the
series saying over a year previously that there would be more posts coming soon. 

i guess im gonna just have to improvise :)

the blog got me through the memory management side of the process so i believe that I should
have a lot more breathing room to implement the features that i want. As of completing the
tutorial, i obviously still dont have access to a standard library, however i can at least
use Vectors and Strings now which are important types, as well as the fact that i have access
to async and heap allocation

## my aims for the project

- whenever i have the chance to work on this project, i want to try and implement a new utility
which could be useful or cool for anyone using the operating system.
  - this could be anything from a cool neofetch style ascii fetcher (if you dont know what im 
talking about, its just a cool ascii logo of the operating system that appears when you open
a terminal sometimes)
- improve the text rendering system to create a set of globally accessible functions and/or macros
in order to render the text in a more visually appealing way to the user (as the default yellow text
does look extremely ugly lmao)
- implement a basic text editor (this will be difficult)
  - i would need a way to move the cursor around the screen and print text at that location
  - this would mean rewriting the majority of the code for the vga buffer module to create a more
  flexible system which allows for applications (modules / commands) to take more direct control of
  the text rendering whenever they are active
