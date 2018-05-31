jmp	0x200
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x23c
jmp	0x7ec
eor	r1, r1
out	0x3f, r1
ldi	r28, 0xFF
out	0x3d, r28
ldi	r29, 0x3F
out	0x3e, r29
ldi	r16, 0x00
out	0x3c, r16
out	0x38, r1
out	0x39, r1
out	0x3a, r1
out	0x3b, r1
ldi	r17, 0x20
ldi	r26, 0x00
ldi	r27, 0x20
ldi	r30, 0xE8
ldi	r31, 0x09
ldi	r16, 0x00
out	0x3b, r16
rjmp	.+4
elpm	r0, Z+
st	X+, r0
cpi	r26, 0x10
cpc	r27, r17
brne	.-10
out	0x3b, r1
call	0x5ee
jmp	0x9e4
jmp	0
push	r28
push	r29
in	r28, 0x3d
in	r29, 0x3e
break
nop
pop	r29
pop	r28
ret
push	r2
push	r3
push	r4
push	r5
push	r6
push	r7
push	r8
push	r9
push	r10
push	r11
push	r12
push	r13
push	r14
push	r15
push	r16
push	r17
push	r28
push	r29
in	r28, 0x3d
in	r29, 0x3e
sbiw	r28, 0x30
out	0x3d, r28
out	0x3e, r29
std	Y+25, r18
std	Y+26, r19
std	Y+27, r20
std	Y+28, r21
std	Y+29, r22
std	Y+30, r23
std	Y+31, r24
std	Y+32, r25
std	Y+33, r10
std	Y+34, r11
std	Y+35, r12
std	Y+36, r13
std	Y+37, r14
std	Y+38, r15
std	Y+39, r16
std	Y+40, r17
ldd	r24, Y+33
std	Y+1, r24
ldd	r24, Y+34
std	Y+2, r24
ldd	r24, Y+35
std	Y+3, r24
ldd	r24, Y+36
std	Y+4, r24
ldd	r24, Y+37
std	Y+5, r24
ldd	r24, Y+38
std	Y+6, r24
ldd	r24, Y+39
std	Y+7, r24
ldd	r24, Y+40
std	Y+8, r24
ldi	r24, 0x01
std	Y+9, r24
std	Y+10, r1
std	Y+11, r1
std	Y+12, r1
std	Y+13, r1
std	Y+14, r1
std	Y+15, r1
std	Y+16, r1
ldd	r24, Y+25
std	Y+17, r24
ldd	r24, Y+26
std	Y+18, r24
ldd	r24, Y+27
std	Y+19, r24
ldd	r24, Y+28
std	Y+20, r24
ldd	r24, Y+29
std	Y+21, r24
ldd	r24, Y+30
std	Y+22, r24
ldd	r24, Y+31
std	Y+23, r24
ldd	r24, Y+32
std	Y+24, r24
ldd	r24, Y+33
std	Y+1, r24
ldd	r24, Y+34
std	Y+2, r24
ldd	r24, Y+35
std	Y+3, r24
ldd	r24, Y+36
std	Y+4, r24
ldd	r24, Y+37
std	Y+5, r24
ldd	r24, Y+38
std	Y+6, r24
ldd	r24, Y+39
std	Y+7, r24
ldd	r24, Y+40
std	Y+8, r24
rjmp	.+480
ldd	r24, Y+1
mov	r25, r24
andi	r25, 0x01
std	Y+41, r25
ldd	r24, Y+2
mov	r25, r24
andi	r25, 0x00
std	Y+42, r25
ldd	r24, Y+3
mov	r25, r24
andi	r25, 0x00
std	Y+43, r25
ldd	r24, Y+4
mov	r25, r24
andi	r25, 0x00
std	Y+44, r25
ldd	r24, Y+5
mov	r25, r24
andi	r25, 0x00
std	Y+45, r25
ldd	r24, Y+6
mov	r25, r24
andi	r25, 0x00
std	Y+46, r25
ldd	r24, Y+7
mov	r25, r24
andi	r25, 0x00
std	Y+47, r25
ldd	r24, Y+8
mov	r25, r24
andi	r25, 0x00
std	Y+48, r25
ldd	r18, Y+41
ldd	r19, Y+42
ldd	r20, Y+43
ldd	r21, Y+44
ldd	r22, Y+45
ldd	r23, Y+46
ldd	r24, Y+47
ldd	r25, Y+48
ldi	r26, 0x00
call	0x9ae
brne	.+2
rjmp	.+168
ldd	r10, Y+17
ldd	r11, Y+18
ldd	r12, Y+19
ldd	r13, Y+20
ldd	r14, Y+21
ldd	r15, Y+22
ldd	r16, Y+23
ldd	r17, Y+24
ldd	r18, Y+9
ldd	r19, Y+10
ldd	r20, Y+11
ldd	r21, Y+12
ldd	r22, Y+13
ldd	r23, Y+14
ldd	r24, Y+15
ldd	r25, Y+16
call	0x71e
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
mov	r2, r10
mov	r3, r11
mov	r4, r12
mov	r5, r13
mov	r6, r14
mov	r7, r15
mov	r8, r16
mov	r9, r17
adiw	r28, 0x07
ldd	r10, Y+63
sbiw	r28, 0x07
adiw	r28, 0x08
ldd	r11, Y+63
sbiw	r28, 0x08
adiw	r28, 0x09
ldd	r12, Y+63
sbiw	r28, 0x09
adiw	r28, 0x0a
ldd	r13, Y+63
sbiw	r28, 0x0a
adiw	r28, 0x0b
ldd	r14, Y+63
sbiw	r28, 0x0b
adiw	r28, 0x0c
ldd	r15, Y+63
sbiw	r28, 0x0c
adiw	r28, 0x0d
ldd	r16, Y+63
sbiw	r28, 0x0d
adiw	r28, 0x0e
ldd	r17, Y+63
sbiw	r28, 0x0e
mov	r18, r2
mov	r19, r3
mov	r20, r4
mov	r21, r5
mov	r22, r6
mov	r23, r7
mov	r24, r8
mov	r25, r9
call	0x7d0
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
std	Y+9, r10
std	Y+10, r11
std	Y+11, r12
std	Y+12, r13
std	Y+13, r14
std	Y+14, r15
std	Y+15, r16
std	Y+16, r17
ldd	r10, Y+17
ldd	r11, Y+18
ldd	r12, Y+19
ldd	r13, Y+20
ldd	r14, Y+21
ldd	r15, Y+22
ldd	r16, Y+23
ldd	r17, Y+24
ldd	r18, Y+17
ldd	r19, Y+18
ldd	r20, Y+19
ldd	r21, Y+20
ldd	r22, Y+21
ldd	r23, Y+22
ldd	r24, Y+23
ldd	r25, Y+24
call	0x71e
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
mov	r2, r10
mov	r3, r11
mov	r4, r12
mov	r5, r13
mov	r6, r14
mov	r7, r15
mov	r8, r16
mov	r9, r17
adiw	r28, 0x07
ldd	r10, Y+63
sbiw	r28, 0x07
adiw	r28, 0x08
ldd	r11, Y+63
sbiw	r28, 0x08
adiw	r28, 0x09
ldd	r12, Y+63
sbiw	r28, 0x09
adiw	r28, 0x0a
ldd	r13, Y+63
sbiw	r28, 0x0a
adiw	r28, 0x0b
ldd	r14, Y+63
sbiw	r28, 0x0b
adiw	r28, 0x0c
ldd	r15, Y+63
sbiw	r28, 0x0c
adiw	r28, 0x0d
ldd	r16, Y+63
sbiw	r28, 0x0d
adiw	r28, 0x0e
ldd	r17, Y+63
sbiw	r28, 0x0e
mov	r18, r2
mov	r19, r3
mov	r20, r4
mov	r21, r5
mov	r22, r6
mov	r23, r7
mov	r24, r8
mov	r25, r9
call	0x7d0
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
std	Y+17, r10
std	Y+18, r11
std	Y+19, r12
std	Y+20, r13
std	Y+21, r14
std	Y+22, r15
std	Y+23, r16
std	Y+24, r17
ldd	r18, Y+1
ldd	r19, Y+2
ldd	r20, Y+3
ldd	r21, Y+4
ldd	r22, Y+5
ldd	r23, Y+6
ldd	r24, Y+7
ldd	r25, Y+8
ldi	r16, 0x01
call	0x972
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
std	Y+1, r10
std	Y+2, r11
std	Y+3, r12
std	Y+4, r13
std	Y+5, r14
std	Y+6, r15
std	Y+7, r16
std	Y+8, r17
ldd	r10, Y+1
ldd	r11, Y+2
ldd	r12, Y+3
ldd	r13, Y+4
ldd	r14, Y+5
ldd	r15, Y+6
ldd	r16, Y+7
ldd	r17, Y+8
mov	r18, r10
mov	r19, r11
mov	r20, r12
mov	r21, r13
mov	r22, r14
mov	r23, r15
mov	r24, r16
mov	r25, r17
ldi	r26, 0x00
call	0x9ae
breq	.+2
rjmp	.-522
ldd	r2, Y+9
ldd	r3, Y+10
ldd	r4, Y+11
ldd	r5, Y+12
ldd	r6, Y+13
ldd	r7, Y+14
ldd	r8, Y+15
ldd	r9, Y+16
adiw	r28, 0x07
ldd	r10, Y+63
sbiw	r28, 0x07
adiw	r28, 0x08
ldd	r11, Y+63
sbiw	r28, 0x08
adiw	r28, 0x09
ldd	r12, Y+63
sbiw	r28, 0x09
adiw	r28, 0x0a
ldd	r13, Y+63
sbiw	r28, 0x0a
adiw	r28, 0x0b
ldd	r14, Y+63
sbiw	r28, 0x0b
adiw	r28, 0x0c
ldd	r15, Y+63
sbiw	r28, 0x0c
adiw	r28, 0x0d
ldd	r16, Y+63
sbiw	r28, 0x0d
adiw	r28, 0x0e
ldd	r17, Y+63
sbiw	r28, 0x0e
mov	r18, r2
mov	r19, r3
mov	r20, r4
mov	r21, r5
mov	r22, r6
mov	r23, r7
mov	r24, r8
mov	r25, r9
call	0x7d0
mov	r2, r18
mov	r3, r19
mov	r4, r20
mov	r5, r21
mov	r6, r22
mov	r7, r23
mov	r8, r24
mov	r9, r25
mov	r10, r2
mov	r11, r3
mov	r12, r4
mov	r13, r5
mov	r14, r6
mov	r15, r7
mov	r16, r8
mov	r17, r9
mov	r18, r10
mov	r19, r11
mov	r20, r12
mov	r21, r13
mov	r22, r14
mov	r23, r15
mov	r24, r16
mov	r25, r17
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
mov	r18, r10
mov	r19, r11
mov	r20, r12
mov	r21, r13
mov	r22, r14
mov	r23, r15
mov	r24, r16
mov	r25, r17
adiw	r28, 0x30
out	0x3d, r28
out	0x3e, r29
pop	r29
pop	r28
pop	r17
pop	r16
pop	r15
pop	r14
pop	r13
pop	r12
pop	r11
pop	r10
pop	r9
pop	r8
pop	r7
pop	r6
pop	r5
pop	r4
pop	r3
pop	r2
ret
push	r2
push	r3
push	r4
push	r5
push	r6
push	r7
push	r8
push	r9
push	r10
push	r11
push	r12
push	r13
push	r14
push	r15
push	r16
push	r17
push	r28
push	r29
in	r28, 0x3d
in	r29, 0x3e
sbiw	r28, 0x08
out	0x3d, r28
out	0x3e, r29
lds	r18, 0x2008
lds	r19, 0x2009
lds	r20, 0x200A
lds	r21, 0x200B
lds	r22, 0x200C
lds	r23, 0x200D
lds	r24, 0x200E
lds	r25, 0x200F
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
lds	r18, 0x2000
lds	r19, 0x2001
lds	r20, 0x2002
lds	r21, 0x2003
lds	r22, 0x2004
lds	r23, 0x2005
lds	r24, 0x2006
lds	r25, 0x2007
mov	r2, r18
mov	r3, r19
mov	r4, r20
mov	r5, r21
mov	r6, r22
mov	r7, r23
mov	r8, r24
mov	r9, r25
mov	r24, r17
push	r24
mov	r24, r16
push	r24
mov	r24, r15
push	r24
mov	r24, r14
push	r24
mov	r24, r13
push	r24
mov	r24, r12
push	r24
mov	r24, r11
push	r24
mov	r24, r10
push	r24
mov	r10, r2
mov	r11, r3
mov	r12, r4
mov	r13, r5
mov	r14, r6
mov	r15, r7
mov	r16, r8
mov	r17, r9
ldi	r18, 0x34
ldi	r19, 0x12
ldi	r20, 0x00
ldi	r21, 0x00
ldi	r22, 0x00
ldi	r23, 0x00
ldi	r24, 0x00
ldi	r25, 0x00
call	0x252
in	r30, 0x3d
in	r31, 0x3e
adiw	r30, 0x08
out	0x3d, r28
out	0x3e, r29
mov	r10, r18
mov	r11, r19
mov	r12, r20
mov	r13, r21
mov	r14, r22
mov	r15, r23
mov	r16, r24
mov	r17, r25
std	Y+1, r10
std	Y+2, r11
std	Y+3, r12
std	Y+4, r13
std	Y+5, r14
std	Y+6, r15
std	Y+7, r16
std	Y+8, r17
call	0x240
ldd	r24, Y+1
ldd	r25, Y+2
adiw	r28, 0x08
out	0x3d, r28
out	0x3e, r29
pop	r29
pop	r28
pop	r17
pop	r16
pop	r15
pop	r14
pop	r13
pop	r12
pop	r11
pop	r10
pop	r9
pop	r8
pop	r7
pop	r6
pop	r5
pop	r4
pop	r3
pop	r2
ret
push	r29
push	r28
push	r17
push	r16
mul	r25, r10
mov	r31, r0
mul	r18, r17
add	r31, r0
mul	r24, r11
add	r31, r0
mul	r24, r10
mov	r30, r0
add	r31, r1
mul	r16, r19
add	r31, r0
mul	r16, r18
add	r30, r0
adc	r31, r1
mul	r20, r14
add	r30, r0
adc	r31, r1
mul	r21, r14
add	r31, r0
mul	r20, r15
add	r31, r0
push	r23
push	r22
push	r11
push	r10
push	r21
push	r20
movw	r26, r10
call	0x9c6
movw	r16, r22
movw	r20, r24
movw	r26, r14
call	0x9c6
movw	r28, r22
add	r30, r24
adc	r31, r25
movw	r26, r12
call	0x7be
pop	r18
pop	r19
movw	r26, r12
call	0x9c6
add	r28, r22
adc	r29, r23
adc	r30, r24
adc	r31, r25
pop	r26
pop	r27
call	0x7be
pop	r18
pop	r19
call	0x9c6
add	r28, r22
adc	r29, r23
adc	r30, r24
adc	r31, r25
movw	r26, r12
call	0x9c6
add	r30, r22
adc	r31, r23
movw	r18, r16
movw	r22, r28
movw	r24, r30
eor	r1, r1
pop	r16
pop	r17
pop	r28
pop	r29
ret
call	0x9c6
add	r20, r22
adc	r21, r23
adc	r28, r24
adc	r29, r25
brcc	.+2
adiw	r30, 0x01
ret
set
rjmp	.+2
clt
mov	r31, r25
or	r31, r17
brmi	.+4
jmp	0x838
ldi	r26, 0x00
ldi	r27, 0x00
ldi	r30, 0xF6
ldi	r31, 0x03
jmp	0x91c
mov	r0, r25
asr	r0
brpl	.+8
call	0x8f0
and	r17, r17
brpl	.+36
ldi	r31, 0x80
eor	r0, r31
ldi	r31, 0xFF
com	r14
com	r15
com	r16
com	r17
com	r11
com	r12
com	r13
neg	r10
sbc	r11, r31
sbc	r12, r31
sbc	r13, r31
sbc	r14, r31
sbc	r15, r31
sbc	r16, r31
sbc	r17, r31
call	0x84e
sbrc	r0, 7
call	0x8f0
in	r28, 0x3d
in	r29, 0x3e
ldi	r30, 0x0C
jmp	0x94e
set
rjmp	.+2
clt
push	r8
push	r9
push	r28
push	r29
call	0x84e
pop	r29
pop	r28
pop	r9
pop	r8
ret
eor	r8, r8
eor	r9, r9
movw	r30, r8
movw	r28, r8
ldi	r27, 0x40
push	r25
eor	r26, r26
cp	r25, r10
cpc	r8, r11
cpc	r9, r12
cpc	r30, r13
cpc	r31, r14
cpc	r28, r15
cpc	r29, r16
cpc	r26, r17
brcc	.+38
mov	r26, r29
mov	r29, r28
mov	r28, r31
mov	r31, r30
mov	r30, r9
mov	r9, r8
mov	r8, r25
mov	r25, r24
mov	r24, r23
mov	r23, r22
mov	r22, r21
mov	r21, r20
mov	r20, r19
mov	r19, r18
eor	r18, r18
subi	r27, 0x08
brne	.-52
pop	r27
rjmp	.+78
mov	r1, r27
pop	r27
eor	r27, r27
add	r18, r18
adc	r19, r19
adc	r20, r20
adc	r21, r21
adc	r22, r22
adc	r23, r23
adc	r24, r24
adc	r25, r25
adc	r8, r8
adc	r9, r9
adc	r30, r30
adc	r31, r31
adc	r28, r28
adc	r29, r29
adc	r26, r26
adc	r27, r27
cp	r8, r10
cpc	r9, r11
cpc	r30, r12
cpc	r31, r13
cpc	r28, r14
cpc	r29, r15
cpc	r26, r16
cpc	r27, r17
brcs	.+18
sub	r8, r10
sbc	r9, r11
sbc	r30, r12
sbc	r31, r13
sbc	r28, r14
sbc	r29, r15
sbc	r26, r16
sbc	r27, r17
ori	r18, 0x01
dec	r1
brne	.-72
brtc	.+10
movw	r18, r8
movw	r20, r30
movw	r22, r28
movw	r24, r26
add	r0, r0
ret
com	r22
com	r23
com	r24
com	r25
com	r19
com	r20
com	r21
neg	r18
sbci	r19, 0xFF
sbci	r20, 0xFF
sbci	r21, 0xFF
sbci	r22, 0xFF
sbci	r23, 0xFF
sbci	r24, 0xFF
sbci	r25, 0xFF
ret
push	r2
push	r3
push	r4
push	r5
push	r6
push	r7
push	r8
push	r9
push	r10
push	r11
push	r12
push	r13
push	r14
push	r15
push	r16
push	r17
push	r28
push	r29
in	r28, 0x3d
in	r29, 0x3e
sub	r28, r26
sbc	r29, r27
out	0x3d, r28
out	0x3e, r29
eijmp
ldd	r2, Y+18
ldd	r3, Y+17
ldd	r4, Y+16
ldd	r5, Y+15
ldd	r6, Y+14
ldd	r7, Y+13
ldd	r8, Y+12
ldd	r9, Y+11
ldd	r10, Y+10
ldd	r11, Y+9
ldd	r12, Y+8
ldd	r13, Y+7
ldd	r14, Y+6
ldd	r15, Y+5
ldd	r16, Y+4
ldd	r17, Y+3
ldd	r26, Y+2
ldd	r27, Y+1
add	r28, r30
adc	r29, r1
out	0x3d, r28
out	0x3e, r29
movw	r28, r26
ret
bst	r25, 7
bld	r1, 0
lsr	r1
sbc	r0, r0
push	r16
cpi	r16, 0x08
brcs	.+38
subi	r16, 0x08
mov	r18, r19
mov	r19, r20
mov	r20, r21
mov	r21, r22
mov	r22, r23
mov	r23, r24
mov	r24, r25
mov	r25, r0
rjmp	.-24
asr	r0
ror	r25
ror	r24
ror	r23
ror	r22
ror	r21
ror	r20
ror	r19
ror	r18
dec	r16
brpl	.-22
pop	r16
ret
eor	r0, r0
sbrc	r26, 7
com	r0
cp	r18, r26
cpc	r19, r0
cpc	r20, r0
cpc	r21, r0
cpc	r22, r0
cpc	r23, r0
cpc	r24, r0
cpc	r25, r0
ret
mul	r26, r18
movw	r22, r0
mul	r27, r19
movw	r24, r0
mul	r26, r19
add	r23, r0
adc	r24, r1
eor	r1, r1
adc	r25, r1
mul	r27, r18
add	r23, r0
adc	r24, r1
eor	r1, r1
adc	r25, r1
ret
cli
rjmp	.-2
