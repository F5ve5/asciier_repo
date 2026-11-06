This project is a pretty simple one

The point is to read pixel data from an image or a screen and then visually recreate that image but only in visual use of character from the ASCII library/collection

More specifically, I'd like to start by reading image data from types like pngs, jpgs and then when I want to move onto recording an entire screen and turning it into
ASCII live it'll be as simple as just running that logic every frame. easy

So for the basic dependencies of this project, the first one would then be something that easily allows me to get pixel data out of an image

And that role I have come to believe is best fulfilled by the Image crate

Yes I'm coding in Rust again

---

As to not squeeze everything out of AI, I will try and look at the docs in order to understand how the Image crate works at first...

The first thing that caught my eye was this line:

let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;

so a new image is being read and stored

ImageReader is reading from something called Cursor::new(bytes)

Apparently Cursor::new(bytes) provides the same reading interface as a file and at least here it takes a Vec<u8> argument

So just as the above line which appears more apparent

let img = ImageReader::open("myimage.png")?.decode()?;

img2 is given a file interface but its just that it is not actually a full file but rather just the same means of interaction against an array of bytes as if it were
a file on any kind

so basically:

    img
    | |
    | |
    \ /
     V
__+__I__T__
|real file|
|_________|

    img2
    | |
    | |
    \ /
     V
__+__I__T__
 fake file

the same interaction

also like the interaction medium itself can be called the "file" in the case of the real file and the "cursor" in case of the fake one

As for why it's called "cursor" and not "fake_file", I think it has to do with that file reading basically revolves around which index in the file it's reading from..
Ayways

---

Moving on image types like png and jpg webp etc. don't appear to be relevant during the execution of the program, the type is decoded into the raw data that the crate
favors before it handles it.

So the actual image format is only relevant when reading or writing an image file

Inside, the collection of pixels is in the Image crate stored as an "ImageBuffer" which itself is a variant of enum "DynamicImage". Each variant represents a different
visual type, so there's like grayscale, RGB and such

So in the earlier analogy, img and img2 would be of enum type DynamicRange and of some ImageBuffer variant initially and convertible between the different variants
later

---

As for why you would want to switch between the different variants, it appears to not only be to directly change the final product, changing between different variants
also appears to make it easier to access certain kinds of data

So for my program I will want to read something similar to the brightness of an image at certain points and converting the image to greyscale might make that easier
through implemented functions for each type and such

-----

This crate happens to implement its' own kind of error type, ImageError which makes it impossible to convert into a std::io::Result<T> which I just recently found out
is an easier variant to write of the underlying std::result::Result<T,E>, there is however another Result variant which isn't only easier to write but also handles
these unique error types and mushes them into the same error type

This Result variant however is not part of the standard library and therefore comes from a crate, a crate called "anyhow" whose main purpose to me apppears to be to
make the error handling system of Rust less strict

I don't however want to import a full crate for such a small issue and especially for something small that I don't understand, I instead invented my own solution that
solves the same problem as anyhow::Result

type Errorr = Box<dyn std::error::Error>;
type Resultt<T> = std::result::Result<T, Errorr>;

Now, the fact happpens to be that anyhow::Result also implements further functionality when it comes to presenting the error as well as providing further backtraced
details but in the matter of handling both ImageError and std::error::Error with the same Result, my solution works just as well.

Two relatively new concepts appear to me in the these two lines, "dyn" which appears to refer to a type that implements a certain type so like:

Box<dyn Food>

would work for HotDog if:

trait Food {...}
struct HotDog
impl Food for HotDog

---

Successfully read image

---

It appears that the DynamicImage variant Luma8 contains only one variable of data for each pixel, that being its' brightness which is presented on a u8 scale

Luma16 presents the same but on a u16 scale and so on

What this means in my code is that if I run .get_pixel(x,y) on a Luma8 image then I will get said scale for that pixel which is exactly what I want for my later
algorithm, as opposed to if I were to run it on an RGB image in which case I would get three different values R, G, B which could be calculated into brightness
but turning it into luma 8 does that for me

---

It seems that writing the image as ascii is surprisingly close already, although I still have to make my own "library" thing of ascii characters that respond to
different levels of brightness and also come up with a small system for generalizing single areas of the image into a single character rather than one for each
pixel.


