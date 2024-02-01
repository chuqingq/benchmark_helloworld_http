package main

import (
    "net/http"
    "fmt"
)

var response = []byte("hello world")

func HandleFunc(res http.ResponseWriter, req * http.Request) {
    // fmt.Fprint(res, "hello world")
    res.Write(response)
}

func main() {
    http.HandleFunc("/", HandleFunc)
    err := http.ListenAndServe(":8081", nil)
    if err != nil {
        fmt.Println(err)
    }
}
