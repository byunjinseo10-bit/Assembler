(ST)
@i
M=0
@KBD
D=M
@ZERO
D;JNE
@WHITE
D;JLE
(ZERO)
    @BLACK
    D=0
    0;JMP
(BLACK)
    @SCREEN
    D=-D
    A=D+A
    M=-1


@i
M=M+1
D=M
@8192
D=D-A
@BLACK
D;JLT
@ST
0;JMP

(WHITE)

@SCREEN
D=-D
A=D+A
M=0


@i
M=M+1
D=M
@8192
D=D-A
@WHITE
D;JLT
@ST
0;JMP