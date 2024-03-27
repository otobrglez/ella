package main

import (
    "C"
	"fmt"
	// "io"
    // "os"
    "strings"
	// dto "github.com/prometheus/client_model/go"
	// "github.com/prometheus/prom2json"
)

/**
- https://gist.github.com/helinwang/2c7bd2867ea5110f70e6431a7c80cd9b
**/

//export add_numbers
func add_numbers(a int, b int) int {
	return a + b
}

//export prom_to_json
func prom_to_json(raw string) *C.char{
    /*
    p := C.getString()
    s := C.GoString(p)
    C.free(unsafe.Pointer(p))
    */

    inUpper := strings.ToUpper(raw)
    return C.CString("Hello " + inUpper)
}

func main() {
    fmt.Println(add_numbers(30,22));
    fmt.Println(C.GoString(prom_to_json("oto brglez")));
 }
