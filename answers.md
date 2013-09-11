Title: Problem Set 1 Answers
Author: Evan Teague

1. User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:23.0) Gecko/20100101 Firefox/23.0

"Mozilla/5.0" represents the current browser that I'm using and the current version. "X11" is the program that creates the GUI and communicates across the network. It is an example of hardware abstraction, something that an OS has to have. "Ubuntu; Linux i686" is the current OS. I would assume that rv:23.0 is the version number of Linux Ubuntu. Gecko/20100101 is what Mozilla Firefox is built on and it handles the layout of the webpage. Firefox/23.0 is the browser and the current version.

2. Rust thinks it's unsafe to modify a global variable like this because visitor_count is beneath the end of the html code, meaning that modifying it above does nothing to the actual variable. 

