#ifndef IDT_H
#define IDT_H

#include "idt_structures.h"

#define INT_VECTOR_INT0  0
#define INT_VECTOR_INT1  1
#define INT_VECTOR_INT2  2
#define INT_VECTOR_INT3  3
#define INT_VECTOR_INT4  4
#define INT_VECTOR_INT5  5
#define INT_VECTOR_INT6  6
#define INT_VECTOR_INT7  7
#define INT_VECTOR_INT8  8
#define INT_VECTOR_INT9  9
#define INT_VECTOR_INT10 10
#define INT_VECTOR_INT11 11
#define INT_VECTOR_INT12 12
#define INT_VECTOR_INT13 13
#define INT_VECTOR_INT14 14

// IDT vectors axle maps IRQs to
#define INT_VECTOR_IRQ0  32
#define INT_VECTOR_IRQ1  33
#define INT_VECTOR_IRQ2  34
#define INT_VECTOR_IRQ3  35
#define INT_VECTOR_IRQ4  36
#define INT_VECTOR_IRQ5  37
#define INT_VECTOR_IRQ6  38
#define INT_VECTOR_IRQ7  39
#define INT_VECTOR_IRQ8  40
#define INT_VECTOR_IRQ9  41
#define INT_VECTOR_IRQ10 42
#define INT_VECTOR_IRQ11 43
#define INT_VECTOR_IRQ12 44
#define INT_VECTOR_IRQ13 45
#define INT_VECTOR_IRQ14 46
#define INT_VECTOR_IRQ15 47

#define INT_VECTOR_IRQ128 128
#define INT_VECTOR_SYSCALL INT_VECTOR_IRQ128

void idt_init(void);
idt_pointer_t* kernel_idt_pointer(void);

#endif
