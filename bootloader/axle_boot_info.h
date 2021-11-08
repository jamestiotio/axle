#ifndef AXLE_BOOT_INFO_H
#define AXLE_BOOT_INFO_H

typedef uint64_t axle_phys_addr_t;
typedef uint64_t axle_virt_addr_t;

typedef enum {
	EFI_MEMORY_RESERVED,
	EFI_LOADER_CODE,
	EFI_LOADER_DATA,
	EFI_BOOT_SERVICES_CODE,
	EFI_BOOT_SERVICES_DATA,
	EFI_RUNTIME_SERVICES_CODE,
	EFI_RUNTIME_SERVICES_DATA,
	EFI_CONVENTIONAL_MEMORY,
	EFI_UNUSABLE_MEMORY,
	EFI_ACPI_RECLAIM_MEMORY,
	EFI_ACPI_MEMORY_NVS,
	EFI_MEMORY_MAPPED_IO,
    EFI_MEMORY_MAPPED_IO_PORT_SPACE,
    EFI_PAL_CODE,
    EFI_MAX_MEMORY_TYPE,
	EFI_MEMORY_TYPE_AXLE_KERNEL_IMAGE = 0x80000000,
	EFI_MEMORY_TYPE_AXLE_INITRD = 0x80000001,
	EFI_MEMORY_TYPE_AXLE_PAGING_STRUCTURE = 0x80000002,
} axle_efi_memory_type_t;

// PT: Note that this must have the same memory layout as efi_memory_descriptor_t,
// as a buffer is casted between the two.
typedef struct {
    uint32_t type;
	uint32_t _pad;
    axle_phys_addr_t phys_start;
	// PT: In QEMU, virt_start is always zero
    axle_virt_addr_t _virt_start;
    uint64_t page_count;
    uint64_t flags;
} axle_efi_memory_descriptor_t;

typedef struct axle_boot_info {
	// Graphics info
	axle_phys_addr_t framebuffer_base;
	uint32_t framebuffer_width;
	uint32_t framebuffer_height;
	uint8_t framebuffer_bytes_per_pixel;

	// Memory map info
	uint64_t memory_map_size;
	uint64_t memory_descriptor_size;
	axle_efi_memory_descriptor_t* memory_descriptors;

	// Kernel info
	uint64_t kernel_string_table_base;
	uint64_t kernel_string_table_size;
	uint64_t kernel_symbol_table_base;
	uint64_t kernel_symbol_table_size;

	// initrd info
	uint64_t initrd_base;
	uint64_t initrd_size;

	uint64_t boot_pml4;
} axle_boot_info_t;

#endif
