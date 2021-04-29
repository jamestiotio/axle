#include "shims.h"

uint32_t html_read_tcp_stream(uint32_t conn_descriptor, uint8_t* buf, uint32_t len) {
	//return net_tcp_conn_read(conn_descriptor, buf, len);
	const char x[] = {72, 84, 84, 80, 47, 49, 46, 49, 32, 52, 48, 48, 32, 66, 97, 100, 32, 82, 101, 113, 117, 101, 115, 116, 10, 68, 97, 116, 101, 58, 32, 84, 117, 101, 44, 32, 51, 48, 32, 77, 97, 114, 32, 50, 48, 50, 49, 32, 48, 48, 58, 48, 52, 58, 49, 52, 32, 71, 77, 84, 10, 83, 101, 114, 118, 101, 114, 58, 32, 65, 112, 97, 99, 104, 101, 10, 67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 56, 57, 48, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 99, 108, 111, 115, 101, 10, 67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 116, 101, 120, 116, 47, 104, 116, 109, 108, 59, 32, 99, 104, 97, 114, 115, 101, 116, 61, 105, 115, 111, 45, 56, 56, 53, 57, 45, 49, 10, 10, 60, 33, 68, 79, 67, 84, 89, 80, 69, 32, 72, 84, 77, 76, 32, 80, 85, 66, 76, 73, 67, 32, 34, 45, 47, 47, 73, 69, 84, 70, 47, 47, 68, 84, 68, 32, 72, 84, 77, 76, 32, 50, 46, 48, 47, 47, 69, 78, 34, 62, 10, 60, 104, 116, 109, 108, 62, 10, 60, 104, 101, 97, 100, 62, 10, 32, 32, 32, 32, 60, 116, 105, 116, 108, 101, 62, 52, 48, 48, 32, 66, 97, 100, 32, 82, 101, 113, 117, 101, 115, 116, 60, 47, 116, 105, 116, 108, 101, 62, 10, 32, 32, 32, 32, 60, 115, 116, 121, 108, 101, 62, 10, 32, 32, 32, 32, 32, 32, 32, 32, 98, 111, 100, 121, 32, 123, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 98, 97, 99, 107, 103, 114, 111, 117, 110, 100, 45, 99, 111, 108, 111, 114, 58, 32, 35, 48, 48, 97, 97, 102, 102, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 125, 10, 32, 32, 32, 32, 32, 32, 32, 32, 104, 49, 32, 123, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 116, 111, 112, 58, 32, 51, 48, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 98, 111, 116, 116, 111, 109, 58, 32, 54, 48, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 108, 101, 102, 116, 58, 32, 50, 48, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 114, 105, 103, 104, 116, 58, 32, 50, 48, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 98, 97, 99, 107, 103, 114, 111, 117, 110, 100, 45, 99, 111, 108, 111, 114, 58, 32, 35, 102, 102, 48, 48, 99, 99, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 102, 111, 110, 116, 45, 115, 105, 122, 101, 58, 32, 52, 101, 109, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 125, 10, 32, 32, 32, 32, 32, 32, 32, 32, 112, 32, 123, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 98, 97, 99, 107, 103, 114, 111, 117, 110, 100, 45, 99, 111, 108, 111, 114, 58, 32, 35, 48, 48, 99, 99, 48, 48, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 98, 111, 116, 116, 111, 109, 58, 32, 56, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 109, 97, 114, 103, 105, 110, 45, 116, 111, 112, 58, 32, 56, 112, 120, 59, 10, 32, 32, 32, 32, 32, 32, 32, 32, 125, 10, 32, 32, 32, 32, 60, 47, 115, 116, 121, 108, 101, 62, 10, 60, 47, 104, 101, 97, 100, 62, 10, 60, 98, 111, 100, 121, 62, 10, 32, 32, 32, 32, 60, 104, 49, 62, 66, 97, 100, 32, 82, 101, 113, 117, 101, 115, 116, 60, 47, 104, 49, 62, 10, 32, 32, 32, 32, 60, 112, 62, 89, 111, 117, 114, 32, 98, 114, 111, 119, 115, 101, 114, 32, 115, 101, 110, 116, 32, 97, 32, 114, 101, 113, 117, 101, 115, 116, 32, 116, 104, 97, 116, 32, 116, 104, 105, 115, 32, 115, 101, 114, 118, 101, 114, 32, 99, 111, 117, 108, 100, 32, 110, 111, 116, 32, 117, 110, 100, 101, 114, 115, 116, 97, 110, 100, 46, 60, 98, 114, 32, 47, 62, 10, 32, 32, 32, 32, 60, 47, 112, 62, 10, 32, 32, 32, 32, 60, 104, 49, 62, 72, 101, 114, 101, 39, 115, 32, 97, 110, 111, 116, 104, 101, 114, 32, 104, 101, 97, 100, 101, 114, 33, 60, 47, 104, 49, 62, 10, 32, 32, 32, 32, 60, 112, 62, 65, 100, 100, 105, 116, 105, 111, 110, 97, 108, 108, 121, 44, 32, 97, 32, 52, 48, 48, 32, 66, 97, 100, 32, 82, 101, 113, 117, 101, 115, 116, 10, 32, 32, 32, 32, 32, 32, 32, 32, 101, 114, 114, 111, 114, 32, 119, 97, 115, 32, 101, 110, 99, 111, 117, 110, 116, 101, 114, 101, 100, 32, 119, 104, 105, 108, 101, 32, 116, 114, 121, 105, 110, 103, 32, 116, 111, 32, 117, 115, 101, 32, 97, 110, 32, 69, 114, 114, 111, 114, 68, 111, 99, 117, 109, 101, 110, 116, 32, 116, 111, 32, 104, 97, 110, 100, 108, 101, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 46, 60, 47, 112, 62, 60, 112, 62, 65, 110, 111, 116, 104, 101, 114, 32, 112, 97, 114, 97, 103, 114, 97, 112, 104, 60, 47, 112, 62, 60, 112, 62, 65, 110, 100, 32, 97, 32, 116, 104, 105, 114, 100, 60, 47, 112, 62, 60, 112, 62, 52, 60, 47, 112, 62, 10, 60, 47, 98, 111, 100, 121, 62, 10, 60, 47, 104, 116, 109, 108, 62};
    memcpy(buf, x, sizeof(x));
	return sizeof(x);
}
