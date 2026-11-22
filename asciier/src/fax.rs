use crate::pref;

pub fn help(){
    println!(
r#"
============================================================

Welcome to ASCIIfy! Print your favorite images or even a live
screen recording as ASCII art.

GETTING STARTED
------------------------------------------------------------

A folder called "{pic_folder}" has been created at:

    {current_dir}

By default, it contains a preview image. Simply place any
images you wish to ASCIIfy inside this folder, then enter 1.


LIVE SCREEN CAPTURE
------------------------------------------------------------

To ASCIIfy a live screen or window, you must launch the
application through PowerShell.

Due to an issue with Windows Capture, the capture functionality
stops working after a single readline operation.

To work around this:

1. Open PowerShell.

2. Navigate to the directory containing the executable:

       cd "{current_dir}"

3. Launch ASCIIfy with the following argument:

       .\asciier.exe 1

4. A Windows interface will appear, allowing you to select
   the screen or window you wish to ASCIIfy.

Do note that more characters per frame and especially colored
characters will make live ASCIIfy noticeably slower.

============================================================
"#,
        pic_folder = pref::PIC_FOLDER_NAME,
        current_dir =  std::env::current_dir().unwrap().display()
    );
}