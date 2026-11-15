use crate::pref;

pub fn help(){
    println!(
        "The following message is shown when the image or config folder is not found or when help is requested:\n
        \n
        Welcome to ASCIIfy, where you can print your favorite images or even a live recording as ASCII art\n
        \n
        A folder has been created at {:?} called {:?}\n
        By default it contains one preview image, put any images you wish to ASCIIfy in it before pressing Enter\n",
        std::env::current_dir().unwrap(), pref::PIC_FOLDER_NAME
        );
    println!();
    println!(
        "In order to ASCIIfy a live screen, you have to use command line invocation,\n
        The reason being that Windows Capture mysteriously abandons functionality after a single readline is used\n
        \n
        So open the terminal and navigate to the directory of the .exe which is probably\n
        {:?}\n
        So enter\n
        cd {:?}\n
        into the terminal and then enter\n
        .\\asciier.exe 2\n
        which will open a windows interface which lets you choose the screen or window that you would like to ASCIIfy™\n",
        std::env::current_dir().unwrap(),std::env::current_dir().unwrap()
        );
    println!();
}