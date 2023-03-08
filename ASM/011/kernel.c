/*
 * C part of Kernel
 */

 void kmain(void) {

    const char *str = "my first Kernel";
    char *vidptr = (char*) 0xb8000; // beginn of video mem
    unsigned int i = 0;
    unsigned int j = 0;

    /*
     * Clear the screen (25 lines with 80 columns each element is 2 Bytes)
     */
     while(j < 80 * 25 * 2) {
        // sets everything to blank
        vidptr[j] = ' ';
        // attribute byte to light gray
        vidptr[j + 1] = 0x07;
        j = j + 2;
     }

     j = 0;

     /*
      * Write Strings to video memory
      */
    while(str[j] != '\0') {
        // set the cahr to show
        vidptr[i] = str[j];
        // set the background to black and forground to gray
        vidptr[i + 1] = 0x07;
        ++j;
        i = i + 2;
    }
    return;
}