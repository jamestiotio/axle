#ifndef AP_BOOTSTRAP_H
#define AP_BOOTSTRAP_H


#define AP_BOOTSTRAP_CODE_PAGE 0x8000
#define AP_BOOTSTRAP_DATA_PAGE 0x9000

#define AP_BOOTSTRAP_PARAM_OFFSET_PROTECTED_MODE_GDT 0x0
#define AP_BOOTSTRAP_PARAM_OFFSET_LONG_MODE_GDT 0x100
#define AP_BOOTSTRAP_PARAM_OFFSET_PML4 0x200
#define AP_BOOTSTRAP_PARAM_OFFSET_IDT 0x300
#define AP_BOOTSTRAP_PARAM_OFFSET_C_ENTRY 0x400
#define AP_BOOTSTRAP_PARAM_OFFSET_STACK_TOP 0x500

#define AP_BOOTSTRAP_PARAM_PROTECTED_MODE_GDT (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_PROTECTED_MODE_GDT)
#define AP_BOOTSTRAP_PARAM_LONG_MODE_GDT (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_LONG_MODE_GDT)
#define AP_BOOTSTRAP_PARAM_PML4 (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_PML4)
#define AP_BOOTSTRAP_PARAM_IDT (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_IDT)
#define AP_BOOTSTRAP_PARAM_C_ENTRY (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_C_ENTRY)
#define AP_BOOTSTRAP_PARAM_STACK_TOP (AP_BOOTSTRAP_DATA_PAGE + AP_BOOTSTRAP_PARAM_OFFSET_STACK_TOP)

#endif