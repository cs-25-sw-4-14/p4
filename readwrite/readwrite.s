.global _main
.text

_main:
    // First file
    mov x16, #5                          // syscall number for open()
    adrp x0, input_filename1@PAGE        // Get the page address
    add x0, x0, input_filename1@PAGEOFF  // Add the offset within the page
    mov x1, #0                           // O_RDONLY
    mov x2, #0                           // Mode (not used for read-only)
    svc #0x80                            // Open syscall
    
    cmp x0, #0                           // Compare return value with 0
    b.lt _exit_error                     // Error handling if file open fails
    
    mov x19, x0                          // Store file descriptor for first file
    
    mov x16, #3                          // syscall number for read()
    mov x0, x19                          // file descriptor
    adrp x1, buffer1@PAGE                // Get buffer address
    add x1, x1, buffer1@PAGEOFF          // Adjust offset
    mov x2, #16                          // Read up to 16 bytes
    svc #0x80                            // Read syscall
    
    cmp x0, #0                           // Check if bytes were read
    b.le _exit_error                     // Exit if no bytes read or error
    
    mov x16, #6                          // syscall number for close()
    mov x0, x19                          // File descriptor
    svc #0x80                            // Close syscall

    mov x21, #0                          // x21 = first number (accumulator)
    adrp x0, buffer1@PAGE                // Get buffer address
    add x0, x0, buffer1@PAGEOFF          // Adjust offset
    mov x22, #0                          // x22 = digit character

_parse_loop1:                            // Start of first parsing loop
    ldrb w22, [x0], #1                   // Load byte and advance pointer
    cmp w22, #'0'                        // Check if character is less than '0'
    blt _done_parsing1                   // If so, stop parsing
    cmp w22, #'9'                        // Check if greater than '9'
    bgt _done_parsing1                   // Stop parsing if non-digit

    sub w22, w22, #'0'                   // Convert ASCII to integer
    mov x20, #10                         // Load constant 10 for multiplication
    mul x21, x21, x20                    // Shift left (multiply by 10)
    add x21, x21, x22                    // Add the current digit
    b _parse_loop1                       // Repeat for next character

_done_parsing1:                          // End of first parsing loop
    mov x24, x21                         // Store first number in x24

    mov x16, #5                          // syscall number for open()
    adrp x0, input_filename2@PAGE        // Get the page address
    add x0, x0, input_filename2@PAGEOFF  // Add the offset within the page
    mov x1, #0                           // O_RDONLY
    mov x2, #0                           // Mode (not used for read-only)
    svc #0x80                            // Open syscall
    
    cmp x0, #0                           // Compare return value with 0
    b.lt _exit_error                     // Error handling if file open fails
    
    mov x19, x0                          // Store file descriptor for second file
    
    // Second file
    mov x16, #3                          // syscall number for read()
    mov x0, x19                          // file descriptor
    adrp x1, buffer2@PAGE                // Get buffer address
    add x1, x1, buffer2@PAGEOFF          // Adjust offset
    mov x2, #16                          // Read up to 16 bytes
    svc #0x80                            // Read syscall
    
    cmp x0, #0                           // Check if bytes were read
    b.le _exit_error                     // Exit if no bytes read or error
    
    mov x16, #6                          // syscall number for close()
    mov x0, x19                          // File descriptor
    svc #0x80                            // Close syscall

    mov x21, #0                          // x21 = second number (accumulator)
    adrp x0, buffer2@PAGE                // Get buffer address
    add x0, x0, buffer2@PAGEOFF          // Adjust offset
    mov x22, #0                          // x22 = digit character

_parse_loop2:                            // Start of second parsing loop
    ldrb w22, [x0], #1                   // Load byte and advance pointer
    cmp w22, #'0'                        // Check if character is less than '0'
    blt _done_parsing2                   // If so, stop parsing
    cmp w22, #'9'                        // Check if greater than '9'
    bgt _done_parsing2                   // Stop parsing if non-digit

    sub w22, w22, #'0'                   // Convert ASCII to integer
    mov x20, #10                         // Load constant 10 for multiplication
    mul x21, x21, x20                    // Shift left (multiply by 10)
    add x21, x21, x22                    // Add the current digit
    b _parse_loop2                       // Repeat for next character

_done_parsing2:                          // End of second parsing loop
    add x21, x21, x24                    // Add first number (x24) to second number (x21)
    
    mov x23, x21                         // Copy result to x23 (for conversion)
    adrp x0, buffer_out@PAGE             // Get buffer address for output
    add x0, x0, buffer_out@PAGEOFF       // Adjust offset
    add x0, x0, #15                      // Move to end (backward storage)
    mov x24, #0                          // Counter for number length

_int_to_str:                             // Start of integer to string conversion
    mov x25, x23                         // Copy current value to x25
    mov x20, #10                         // Load constant 10 for division
    udiv x23, x23, x20                   // x23 = x23 / 10 (quotient)
    msub x25, x23, x20, x25              // x25 = x25 - (x23 * x20) = remainder
    add x25, x25, #'0'                   // Convert remainder to ASCII
    strb w25, [x0], #-1                  // Store byte and move left
    add x24, x24, #1                     // Increment character count
    cmp x23, #0                          // Check if quotient is zero
    b.ne _int_to_str                     // If not, continue conversion

    add x0, x0, #1                       // Adjust buffer to start of number
    mov x25, x0                          // Store start address in x25

    mov x16, #5                          // syscall number for open()
    adrp x0, output_filename@PAGE        // Get the page address
    add x0, x0, output_filename@PAGEOFF  // Add the offset within the page
    mov x1, #0x201                       // O_CREAT | O_WRONLY | O_TRUNC
    mov x2, #0x1A4                       // 0644 permission
    svc #0x80                            // Open syscall
    
    cmp x0, #0                           // Compare return value with 0
    b.lt _exit_error                     // Error handling if file open fails
    
    mov x19, x0                          // Store file descriptor

    mov x16, #4                          // syscall number for write()
    mov x0, x19                          // file descriptor
    mov x1, x25                          // pointer to data
    mov x2, x24                          // Number of bytes to write
    svc #0x80                            // Write syscall

    mov x16, #6                          // syscall number for close()
    mov x0, x19                          // file descriptor
    svc #0x80                            // Close syscall

    mov x0, 0                            // Return code 0 (success)
    mov x16, #1                          // syscall number for exit()
    svc #0x80                            // Exit syscall

_exit_error:                             // Error handling label
    mov x0, #1                           // Return code 1 (error)
    mov x16, #1                          // syscall number for exit()
    svc #0x80                            // Exit syscall

.section __DATA,__data                   // Start of data section
input_filename1:                         // Label for first input filename
    .asciz "input1.txt"                  // Null-terminated string constant

input_filename2:                         // Label for second input filename
    .asciz "input2.txt"                  // Null-terminated string constant

output_filename:                         // Label for output filename
    .asciz "output.txt"                  // Null-terminated string constant

buffer1:                                 // Buffer for first input
    .zero 16                             // Reserve 16 bytes initialized to zero

buffer2:                                 // Buffer for second input
    .zero 16                             // Reserve 16 bytes initialized to zero

buffer_out:                              // Buffer for output
    .zero 16                             // Reserve 16 bytes initialized to zero
