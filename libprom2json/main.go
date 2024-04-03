package main

import (
    "C"
	"fmt"
	"io"
    "os"
    "strings"
	dto "github.com/prometheus/client_model/go"
	"github.com/prometheus/prom2json"
	"encoding/json"
)

type Result struct {
	JsonText string
	Err      error
}

type ResultRaw struct {
	Families []*prom2json.Family
	Err      error
}

//export prom_to_json
func prom_to_json(raw string) (*C.char, *C.char){
    mfChan := make(chan *dto.MetricFamily, 1024)
    errChan := make(chan Result, 1)
	reader := strings.NewReader(raw)

    go func() {
        var result Result
        result.JsonText, result.Err = parseMetrics(reader, mfChan)
        errChan <- result
    }()

    result := []*prom2json.Family{}
	for mf := range mfChan {
		result = append(result, prom2json.NewFamily(mf))
	}

	errResult := <-errChan

    if errResult.Err != nil {
        return C.CString("[]"), C.CString(errResult.Err.Error())
    }

	jsonText, err := json.Marshal(result)
    if err != nil {
        return C.CString("[]"), C.CString(err.Error())
    }

    return C.CString(string(jsonText)), C.CString("")
}

func parseMetrics(reader io.Reader, mfChan chan<- *dto.MetricFamily) (string, error) {
	err := prom2json.ParseReader(reader, mfChan)
	if err != nil {
		return "", fmt.Errorf("Reading metrics failed with: %w", err)
	}
	return "", nil
}

/*

//export prom_to_json_raw
func prom_to_json_raw(raw string) ([]*prom2json.Family, *C.char){
    mfChan := make(chan *dto.MetricFamily, 1024)
    errChan := make(chan ResultRaw, 1)
	reader := strings.NewReader(raw)

    go func() {
        var result ResultRaw
        result.Families, result.Err = parseMetricsRaw(reader, mfChan)
        errChan <- result
    }()

    result := []*prom2json.Family{}
	for mf := range mfChan {
		result = append(result, prom2json.NewFamily(mf))
	}

	errResult := <-errChan

    if errResult.Err != nil {
        return result, C.CString(errResult.Err.Error())
    }

    return result, C.CString("")
}

func parseMetricsRaw(reader io.Reader, mfChan chan<- *dto.MetricFamily) ([]*prom2json.Family, error) {
    result := []*prom2json.Family{}
	err := prom2json.ParseReader(reader, mfChan)
	if err != nil {
		return result, fmt.Errorf("Reading metrics failed with: %w", err)
	}
	return result, nil
}

*/

func main() {
    could_be_result, fail_as_cstring := prom_to_json("# oto brglez")
    fmt.Println(C.GoString(could_be_result))
    fmt.Println(C.GoString(fail_as_cstring))
    fmt.Println("---")

    // From file
    prom, err := os.ReadFile("./tests/data/example.prom")
    if err != nil {
        fmt.Print(err)
    }

    // fmt.Println(b) bytes
    prom_as_string := string(prom)
    result_as_cstring, _ := prom_to_json(prom_as_string)
    result := C.GoString(result_as_cstring)
    fmt.Println(result)
 }
