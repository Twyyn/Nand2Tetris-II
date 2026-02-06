// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

//// Replace this comment with your code.

(LOOP)
  @KBD
  D=M
  @BLACK
  D;JGT
  @WHITE
  0;JMP

(BLACK)
  @R2
  M=-1
  @FILL
  0;JMP

(WHITE)
  @R2
  M=0

(FILL)
  @SCREEN
  D=A
  @R1
  M=D

(FILL_LOOP)
  @R2
  D=M
  @R1
  A=M
  M=D

  @R1
  M=M+1

  @8192
  D=A
  @SCREEN
  D=D+A
  @R1
  D=M-D
  @FILL_LOOP
  D;JLT

(END)
  @LOOP
  0;JMP