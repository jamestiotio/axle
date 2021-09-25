#include <stddef.h>

#include <image_viewer/image_viewer_messages.h>

#include "util.h"

void print_tabs(uint32_t count) {
	for (uint32_t i = 0; i < count; i++) {
		putchar('\t');
	}
}

void print_fs_tree(fs_node_t* node, uint32_t depth) {
	print_tabs(depth);
	const char* type = node->base.is_directory ? "Dir" : "File";
	printf("<%s %s", type, node->base.name);
	if (node->base.type == FS_NODE_TYPE_INITRD) {
		printf(", Start = 0x%08x, Len = 0x%08x>\n", node->initrd.initrd_offset, node->initrd.size);
	}
	else if (node->base.type == FS_NODE_TYPE_FAT) {
		printf(", FirstSector %ld>\n", node->fat.first_fat_entry_idx_in_file);
	}
	else if (node->base.type == FS_NODE_TYPE_ROOT) {
		printf(" (Root)>\n");
	}
	else if (node->base.type == FS_NODE_TYPE_BASE) {
		printf(">\n");
	}
	else {
		printf("Node type: %ld\n", node->base.type);
		assert(false, "Unknown fs node type");
	}

	if (node->base.children) {
		for (uint32_t i = 0; i < node->base.children->size; i++) {

			fs_node_t* child = array_lookup(node->base.children, i);
			print_fs_tree(child, depth + 1);
		}
	}
}

uint32_t depth_first_search__idx(fs_base_node_t* parent, fs_base_node_t* find, uint32_t sum, bool* out_found) {
	sum += 1;
	if (parent == find) {
		*out_found = true;
		return sum;
	}

	if (parent->children) {
		for (uint32_t i = 0; i < parent->children->size; i++) {
			fs_base_node_t* child = array_lookup(parent->children, i);
			sum = depth_first_search__idx(child, find, sum, out_found);

			if (*out_found) {
				return sum;
			}
		}
	}
	return sum;
}

bool str_ends_with(char* str, char* suffix) {
	str = strchr(str, '.');
	if (str) {
		return !strcmp(str, suffix);
	}
	return false;
}

bool str_ends_with_any(char* str, char* suffixes[]) {
	for (uint32_t i = 0; i < 32; i++) {
		if (suffixes[i] == NULL) {
			return false;
		}
		if (str_ends_with(str, suffixes[i])) {
			return true;
		}
	}
	assert(false, "Failed to find NULL terminator");
	return false;
}

void launch_amc_service_if_necessary(const char* service_name) {
	if (amc_service_is_active(service_name)) {
		printf("Will not launch %s because it's already active!\n");
		return;
	}

	const char* program_name = NULL;
	if (!strncmp(service_name, IMAGE_VIEWER_SERVICE_NAME, AMC_MAX_SERVICE_NAME_LEN)) {
		program_name = "image_viewer";
	}
	else {
		assert(false, "Unknown service name");
	}

	initrd_fs_node_t* node = vfs_find_node_by_name(program_name);
	assert(node != NULL, "Failed to find FS node");
	vfs_launch_program_by_node(node);
}