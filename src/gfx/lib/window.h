#ifndef WINDOW_H
#define WINDOW_H

#include <std/std_base.h>
#include <stdint.h>
#include "rect.h"
#include "ca_layer.h"
#include "color.h"
#include "view.h"

__BEGIN_DECLS

typedef struct window {
	//common
	Rect frame;
	char needs_redraw;
	ca_layer* layer;
	struct window* superview;
	array_m* subviews;

	Size size;
	char* title;
	struct view* title_view;
	struct view* content_view;
	Color border_color;
	int border_width;
} Window;

Window* create_window(Rect frame);
void window_teardown(Window* window);

void add_subwindow(Window* window, Window* subwindow);
void remove_subwindow(Window* window, Window* subwindow);

//add window to desktop hierarchy
void present_window(Window* window);
//remove window from desktop hierarchy
//should this also free window resources? What if window should be presented again later?
void kill_window(Window* window);

void set_border_width(Window* window, int width);

__END_DECLS

#endif