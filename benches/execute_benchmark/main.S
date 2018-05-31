jmp	0x1fc
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
jmp	0x21c
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
call	0x220
jmp	0x35c
jmp	0
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
sbiw	r28, 0x10
out	0x3d, r28
out	0x3e, r29
std	Y+1, r1
std	Y+2, r1
std	Y+3, r1
std	Y+4, r1
std	Y+5, r1
std	Y+6, r1
std	Y+7, r1
std	Y+8, r1
std	Y+9, r1
std	Y+10, r1
std	Y+11, r1
std	Y+12, r1
std	Y+13, r1
std	Y+14, r1
std	Y+15, r1
std	Y+16, r1
rjmp	.+122
ldd	r18, Y+1
ldd	r19, Y+2
ldd	r20, Y+3
ldd	r21, Y+4
ldd	r22, Y+5
ldd	r23, Y+6
ldd	r24, Y+7
ldd	r25, Y+8
ldd	r10, Y+9
ldd	r11, Y+10
ldd	r12, Y+11
ldd	r13, Y+12
ldd	r14, Y+13
ldd	r15, Y+14
ldd	r16, Y+15
ldd	r17, Y+16
call	0x332
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
ldd	r18, Y+9
ldd	r19, Y+10
ldd	r20, Y+11
ldd	r21, Y+12
ldd	r22, Y+13
ldd	r23, Y+14
ldd	r24, Y+15
ldd	r25, Y+16
ldi	r26, 0x01
call	0x344
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
ldd	r10, Y+9
ldd	r11, Y+10
ldd	r12, Y+11
ldd	r13, Y+12
ldd	r14, Y+13
ldd	r15, Y+14
ldd	r16, Y+15
ldd	r17, Y+16
mov	r18, r10
mov	r19, r11
mov	r20, r12
mov	r21, r13
mov	r22, r14
mov	r23, r15
mov	r24, r16
mov	r25, r17
cpi	r18, 0x7F
sbci	r19, 0x84
sbci	r20, 0x1E
cpc	r21, r1
cpc	r22, r1
cpc	r23, r1
cpc	r24, r1
cpc	r25, r1
breq	.+2
brcc	.+2
rjmp	.-176
break
ldd	r24, Y+1
ldd	r25, Y+2
adiw	r28, 0x10
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
ret
add	r18, r10
adc	r19, r11
adc	r20, r12
adc	r21, r13
adc	r22, r14
adc	r23, r15
adc	r24, r16
adc	r25, r17
ret
eor	r0, r0
sbrc	r26, 7
com	r0
add	r18, r26
adc	r19, r0
adc	r20, r0
adc	r21, r0
adc	r22, r0
adc	r23, r0
adc	r24, r0
adc	r25, r0
ret
cli
rjmp	.-2
