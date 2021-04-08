#include "elem_stack.h"
#include <stdint.h>
#include <stdlib.h>

elem_stack_t* stack_create(void) {
	elem_stack_t* s = calloc(1, sizeof(elem_stack_t));
	return s;
}

void stack_destroy(elem_stack_t* s) {
	free(s);
}

void stack_push(elem_stack_t* stack, void* item) {
	// Set up the entry
	stack_elem_t* elem = calloc(1, sizeof(stack_elem_t));
	elem->payload = item;

	if (!stack->head) {
		stack->head = elem;
	}
	else if (stack->tail) {
		// Set up the `next` pointer of the current tail
		elem->prev = stack->tail;
		stack->tail->next = elem;
	}

	stack->count += 1;
	stack->tail = elem;
}

void* stack_peek(elem_stack_t* stack) {
	if (!stack->count) {
		return NULL;
	}
	return stack->tail->payload;
}

void* stack_pop(elem_stack_t* stack) {
	if (!stack->count) {
		return NULL;
	}

	stack_elem_t* last_elem = stack->tail;
	stack->count -= 1;

	if (stack->count == 0) {
		stack->head = NULL;
		stack->tail = NULL;
	}
	else {
		stack_elem_t* new_last = last_elem->prev;
		stack->tail = new_last;
		new_last->next = NULL;
	}

	void* ret = last_elem->payload;
	free(last_elem);
	return ret;
}

void* stack_get(elem_stack_t* stack, uint32_t idx) {
	if (!stack->count || idx > stack->count || idx < 0) {
		return NULL;
	}
	stack_elem_t* ptr = stack->head;
	for (uint32_t i = 1; i < idx; i++) {
		ptr = ptr->next;
	}
	return ptr->payload;
}
