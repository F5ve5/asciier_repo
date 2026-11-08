## **ASCIIer**

A fun little project that lets you turn images into ASCII characters in the terminal

Just select one of the images or your own from the folder inside of the terminal interface and then continue to configure how they are to be processed

### **How does it work?**

The image capturing and processing is done using the Image crate, which makes support for pretty much all known image formats possible

So I convert the image into basically a 2d array of brightness values and then iterate over those values in intervals to turn each one into its' equivalent character

### **What's missing?**

In the future I plan to add the ability to "ASCIIfy" a live OBS-ish recording right in the terminal

I also plan to add the ability to add visual ASCII effects to the images and process them into their own image files or even gifs

<img width="828" height="1200" alt="579cd4782ede76d15aa8857691a999fa" src="https://github.com/user-attachments/assets/0f60e13d-ca0e-40f8-86df-90d3505e3aa1" />
<img width="556" height="982" alt="Skärmbild 2026-11-07 135107" src="https://github.com/user-attachments/assets/482458b9-a34d-40d9-9f81-c92b8922121e" />
