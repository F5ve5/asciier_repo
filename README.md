## **ASCIIer**

A fun little project that lets you turn images into ASCII characters in the terminal

Just select one of the images or your own from the folder inside of the terminal interface and then continue to configure how they are to be processed

Or if you instead want to asciify a live input using windows capture then launch the exe with an argument as instructed when first running the exe normally

### **How does it work?**

The image capturing and processing is done using the Image crate, which makes support for pretty much all known image formats possible

So the image is converted into basically a 2d array of brightness values and then iterate over those values in intervals to turn each one into its' equivalent character

The live ascii-ing (turning a live wincap recording into ascii art) works in a similar way, except that another thread is created to handle the processing and subsequently requests a new frame from the main thread that gets dedicated to wincap

### **What's missing?**

In the future I plan to add the ability to add visual ASCII effects to images and possibly streams and process them into their own image files or even gifs

Besides that, the main focus will be to make the ascii stream less visually straining to look at

### **How do you run it?**

Just press the exe and then follow the instructions

<img width="556" height="982" alt="Skärmbild 2026-11-07 135107" src="https://github.com/user-attachments/assets/482458b9-a34d-40d9-9f81-c92b8922121e" />

<img width="2558" height="1383" alt="Screenshot 2026-11-15 235533" src="https://github.com/user-attachments/assets/fcb572cd-07c6-46bc-a586-b39c16095b37" />
