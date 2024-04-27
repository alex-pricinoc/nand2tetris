// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.


(START)
    @SCREEN
    D=A
    @R0
    M=D // save screen start in R0
(READ)
    @KBD
    D=M
    @BLACK
    D;JGT
    @WHITE
    D;JEQ
(WHITE)
    @R1
    M=0 // save color in R1
    @DRAW
    0;JMP
(BLACK)
    @R1
    M=-1
    @DRAW
    0;JMP
(DRAW)
    // set pixels at RAM[R0] to color at R1
    @R1
    D=M
    @R0
    A=M
    M=D

    // increment address
    @R0
    MD=M+1

    @KBD
    D=A-D // if (KBD-SCREEN = 0) exit
    @START
    D;JEQ

    @READ
    0;JMP
