/**
 * Interrupt Service Routine wrappers
*/

.text

.macro isr_wrapper vector error_code=0
	.global _isr_wrapper_\vector
	.type _isr_wrapper_\vector, @function
	_isr_wrapper_\vector:
		.if !\error_code
			pushl $0x0
		.endif
		pushl $\vector
		jmp x86_interrupt_wrapper
.endm

.global x86_interrupt_wrapper
.type x86_interrupt_wrapper @function
x86_interrupt_wrapper:
	/* Stack: upper -> lower
		(ss) // 72
		(esp) // 68
		eflags // 64
		cs // 60
		eip // 56
		error code // 52
		vector // 48 // pushed by isr_wrapper
		esp // 44
		gs // 40
		fs // 36
		es // 32
		ds // 28
		eax // 24
		ebx // 20
		ecx // 16
		edx // 12
		esi // 8
		edi // 4
		ebp // 0
	*/
	cld
	pushl %esp
	subl $16, %esp
	pushl %eax
	xor %eax, %eax
	movl %eax, 16(%esp)
	movl %eax, 12(%esp)
	movl %eax, 8(%esp)
	movl %eax, 4(%esp)
	movw %gs, 16(%esp)
	movw %fs, 12(%esp)
	movw %es, 8(%esp)
	movw %ds, 4(%esp)
	pushl %ebx
	pushl %ecx
	pushl %edx
	pushl %esi
	pushl %edi
	pushl %ebp

	movl 60(%esp), %eax
	andl $0b11, %eax
	jz .L1

	// set core data segment
	movl $0x10, %eax
	movl %ax, %es
	movl %ax, %ds
	movl %ax, %fs
	movl %ax, %gs

	/// @todo save x84FPU state

	.L1:
	movl %esp, %eax
	call x86_interrupt_handler

.global x86_interrupt_ret
.type x86_interrupt_ret @function
x86_interrupt_ret:
	popl %ebp
	popl %edi
	popl %esi
	popl %edx
	popl %ecx
	popl %ebx
	popl %eax
	movw (%esp), %ds
	movw 4(%esp), %es
	movw 8(%esp), %fs
	movw 12(%esp), %gs
	addl $16, %esp
	popl %esp
	addl $8, %esp // skip vector and error code

	pushl %eax
	movl 8(%esp), %eax // check CS
	andl $0b11, %eax
	jz .L2
	// todo: cross-ring return
	.L2:
	popl %eax
	iret

isr_wrapper 0
isr_wrapper 1
isr_wrapper 2
isr_wrapper 3
isr_wrapper 4
isr_wrapper 5
isr_wrapper 6
isr_wrapper 7
isr_wrapper 8, error_code=1
isr_wrapper 9
isr_wrapper 10, error_code=1
isr_wrapper 11, error_code=1
isr_wrapper 12, error_code=1
isr_wrapper 13, error_code=1
isr_wrapper 14, error_code=1
isr_wrapper 15
isr_wrapper 16
isr_wrapper 17, error_code=1
isr_wrapper 18
isr_wrapper 19
isr_wrapper 20
isr_wrapper 21, error_code=1
isr_wrapper 22
isr_wrapper 23
isr_wrapper 24
isr_wrapper 25
isr_wrapper 26
isr_wrapper 27
isr_wrapper 28
isr_wrapper 29
isr_wrapper 30
isr_wrapper 31
isr_wrapper 32
isr_wrapper 33
isr_wrapper 34
isr_wrapper 35
isr_wrapper 36
isr_wrapper 37
isr_wrapper 38
isr_wrapper 39
isr_wrapper 40
isr_wrapper 41
isr_wrapper 42
isr_wrapper 43
isr_wrapper 44
isr_wrapper 45
isr_wrapper 46
isr_wrapper 47
isr_wrapper 48
isr_wrapper 49
isr_wrapper 50
isr_wrapper 51
isr_wrapper 52
isr_wrapper 53
isr_wrapper 54
isr_wrapper 55
isr_wrapper 56
isr_wrapper 57
isr_wrapper 58
isr_wrapper 59
isr_wrapper 60
isr_wrapper 61
isr_wrapper 62
isr_wrapper 63
isr_wrapper 64
isr_wrapper 65
isr_wrapper 66
isr_wrapper 67
isr_wrapper 68
isr_wrapper 69
isr_wrapper 70
isr_wrapper 71
isr_wrapper 72
isr_wrapper 73
isr_wrapper 74
isr_wrapper 75
isr_wrapper 76
isr_wrapper 77
isr_wrapper 78
isr_wrapper 79
isr_wrapper 80
isr_wrapper 81
isr_wrapper 82
isr_wrapper 83
isr_wrapper 84
isr_wrapper 85
isr_wrapper 86
isr_wrapper 87
isr_wrapper 88
isr_wrapper 89
isr_wrapper 90
isr_wrapper 91
isr_wrapper 92
isr_wrapper 93
isr_wrapper 94
isr_wrapper 95
isr_wrapper 96
isr_wrapper 97
isr_wrapper 98
isr_wrapper 99
isr_wrapper 100
isr_wrapper 101
isr_wrapper 102
isr_wrapper 103
isr_wrapper 104
isr_wrapper 105
isr_wrapper 106
isr_wrapper 107
isr_wrapper 108
isr_wrapper 109
isr_wrapper 110
isr_wrapper 111
isr_wrapper 112
isr_wrapper 113
isr_wrapper 114
isr_wrapper 115
isr_wrapper 116
isr_wrapper 117
isr_wrapper 118
isr_wrapper 119
isr_wrapper 120
isr_wrapper 121
isr_wrapper 122
isr_wrapper 123
isr_wrapper 124
isr_wrapper 125
isr_wrapper 126
isr_wrapper 127
isr_wrapper 128
isr_wrapper 129
isr_wrapper 130
isr_wrapper 131
isr_wrapper 132
isr_wrapper 133
isr_wrapper 134
isr_wrapper 135
isr_wrapper 136
isr_wrapper 137
isr_wrapper 138
isr_wrapper 139
isr_wrapper 140
isr_wrapper 141
isr_wrapper 142
isr_wrapper 143
isr_wrapper 144
isr_wrapper 145
isr_wrapper 146
isr_wrapper 147
isr_wrapper 148
isr_wrapper 149
isr_wrapper 150
isr_wrapper 151
isr_wrapper 152
isr_wrapper 153
isr_wrapper 154
isr_wrapper 155
isr_wrapper 156
isr_wrapper 157
isr_wrapper 158
isr_wrapper 159
isr_wrapper 160
isr_wrapper 161
isr_wrapper 162
isr_wrapper 163
isr_wrapper 164
isr_wrapper 165
isr_wrapper 166
isr_wrapper 167
isr_wrapper 168
isr_wrapper 169
isr_wrapper 170
isr_wrapper 171
isr_wrapper 172
isr_wrapper 173
isr_wrapper 174
isr_wrapper 175
isr_wrapper 176
isr_wrapper 177
isr_wrapper 178
isr_wrapper 179
isr_wrapper 180
isr_wrapper 181
isr_wrapper 182
isr_wrapper 183
isr_wrapper 184
isr_wrapper 185
isr_wrapper 186
isr_wrapper 187
isr_wrapper 188
isr_wrapper 189
isr_wrapper 190
isr_wrapper 191
isr_wrapper 192
isr_wrapper 193
isr_wrapper 194
isr_wrapper 195
isr_wrapper 196
isr_wrapper 197
isr_wrapper 198
isr_wrapper 199
isr_wrapper 200
isr_wrapper 201
isr_wrapper 202
isr_wrapper 203
isr_wrapper 204
isr_wrapper 205
isr_wrapper 206
isr_wrapper 207
isr_wrapper 208
isr_wrapper 209
isr_wrapper 210
isr_wrapper 211
isr_wrapper 212
isr_wrapper 213
isr_wrapper 214
isr_wrapper 215
isr_wrapper 216
isr_wrapper 217
isr_wrapper 218
isr_wrapper 219
isr_wrapper 220
isr_wrapper 221
isr_wrapper 222
isr_wrapper 223
isr_wrapper 224
isr_wrapper 225
isr_wrapper 226
isr_wrapper 227
isr_wrapper 228
isr_wrapper 229
isr_wrapper 230
isr_wrapper 231
isr_wrapper 232
isr_wrapper 233
isr_wrapper 234
isr_wrapper 235
isr_wrapper 236
isr_wrapper 237
isr_wrapper 238
isr_wrapper 239
isr_wrapper 240
isr_wrapper 241
isr_wrapper 242
isr_wrapper 243
isr_wrapper 244
isr_wrapper 245
isr_wrapper 246
isr_wrapper 247
isr_wrapper 248
isr_wrapper 249
isr_wrapper 250
isr_wrapper 251
isr_wrapper 252
isr_wrapper 253
isr_wrapper 254
isr_wrapper 255
