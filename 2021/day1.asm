.section .data
	input: .asciz "199
200
208
210
200
207
240
269
260
263"

// -------Test data-------
// 200 Increase
// 208 Increase
// 210 Increase
// 200 Decrease
// 207 Increase
// 240 Increase
// 269 Increase
// 260 Decrease
// 263 Increase
// -----------------------
// Expected count = 7
	
.section .text 
.global _start
// \r Ascii value 13
// \n Ascii value 10
// E.g. num = 199 -> 199\r\n
// Bytes: 1, 9, 9, 13, 10, Next_number
_start:
	// Loads input
	ldr r0, =input 
	// Resets registers
	mov r1, #0 				// Byte counter
	mov r2, #0				// Current digit
	mov r3, #0 				// Current number
	mov r4, #0 				// Previous number
	mov r5, #0 				// Count of increases
	mov r6, #0 				// Status byte, Reading number / finished reading
	mov r7, #10 			// Used to shift digit by base 10
// Loop over each byte
loop:
	ldrb r2, [r0, r1]		// Load byte at r1 from input in r2
	cmp r2, #0				// Compare for string-terminater
	beq endloop 			// Equal, branch endloop
	if:
		cmp r2, #13 		// Compare for \r
		bne else 			// Not equal, branch else
		cmp r3, r4  		// Compare previous number with current number
		ble decreasing		// Less or equal, branch decreasing
		cmp r4, #0  		// For first iteration when previous number is 0
		beq decreasing 		// No increase for first iteration
		add r5, r5, #1 		// Increasing, count += 1
		decreasing:
			mov r4, r3  	// Store current number as previous
			mov r3, #0  	// Reset register
			add r1, r1, #1	// Skips 1 byte, \n
			mov r6, #0		// Finished reading number
			b endif
	else:
		// Ascii value for 0 = 48
		sub r2, r2, #48 	// Gets the actual number
		cmp r6, #1	 		// Compares the reading status byte
		bne add_num  		// Not equal, branch add number
		mul r3, r3, r7  	// multiplies current number with 10
		add_num:
			add r3, r3, r2	// Adds current digit to current number
			mov r6, #1 		// Set statys byte to "reading number"
	endif:
		add r1, r1, #1 		// Counter +=1 for the next byte
		b loop
endloop:
	// Final case where the lest number is larger then the previous number
	cmp r3, r4  			// Compares current number with previous
	ble end  				// Less or equal, branch to end
	add r5, r5, #1 			// Increase, count += 1
end:
	mov r0, r5				// Return count
	b . 					// Branch unconditionally to self
